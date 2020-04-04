use std::collections::HashMap;

use yolol_number::prelude::*;

use crate::grammar::ast::{ InnerStatement, OuterStatement, Expression };
use crate::error::{ CompilerError };
use crate::yolol;
use crate::compiler::typecheck::{ Type, infer_expr_type, type_check_assignment };
use crate::compiler::calls::{ CallType };
use super::initial_blocks::{ InitialStatementBlocks, Block };
use super::super::fields::{ canonicalise_field_path };


#[derive(Debug)]
pub enum YololBlock {
    // A set of statements that execute in the given order
    Statements(Option<String>, Vec<yolol::ast::Statement>),

    // A set of statements which must be put onto a single line of output
    Line(Option<String>, Vec<yolol::ast::Statement>)
}

#[derive(Debug)]
pub struct YololStatementBlocks {
    pub blocks: Vec<YololBlock>,

    pub types: HashMap<String, Type>,
    pub consts: HashMap<String, yolol::ast::Expression>
}

impl InitialStatementBlocks {

    pub fn covert_yolol_blocks(self) -> Result<YololStatementBlocks, CompilerError> {

        let mut types = HashMap::new();
        let mut consts = HashMap::new();
        let result: Result<_, _> = self.blocks
            .iter()
            .map(|x| handle_block(x, &mut types, &mut consts))
            .collect();
        
        return Ok(YololStatementBlocks {
            blocks: result?,
            types: types,
            consts: consts
        });

        fn handle_block(b: &Block, types: &mut HashMap<String, Type>, consts: &mut HashMap<String, yolol::ast::Expression>) -> Result<YololBlock, CompilerError> {
            match b {
                Block::Statements(name, stmts) => Ok(YololBlock::Statements(name.clone(), handle_outer_stmts(stmts, types, consts)?)),
                Block::Line(label, stmts) => Ok(YololBlock::Line(label.clone(), handle_inner_stmts(stmts, types, consts)?))
            }
        }

        fn handle_outer_stmts(stmts: &Vec::<OuterStatement>, types: &mut HashMap<String, Type>, consts: &mut HashMap<String, yolol::ast::Expression>) -> Result<Vec<yolol::ast::Statement>, CompilerError> {
            Ok(stmts
                .iter()
                .map(|x| handle_outer_stmt(x, types, consts))
                .collect::<Result<Vec<_>, CompilerError>>()?
                .into_iter()
                .flatten()
                .collect()
            )
        }

        fn handle_outer_stmt(inner: &OuterStatement, types: &mut HashMap<String, Type>, consts: &mut HashMap<String, yolol::ast::Expression>) -> Result<Vec<yolol::ast::Statement>, CompilerError> {
            match inner {
                OuterStatement::Inner(inner) => handle_inner_stmt(&inner, types, consts),

                // There should be no `Label` statements here, they've been promoted into named blocks by the previous pass
                OuterStatement::Label(name) => panic!("Encountered label `{}` as an outer statement (6fe60057-c45d-4a2b-882d-308cd141d4e1)", name),

                // There should be no `Line` statements here, they've been separated into Line blocks by the previous pass
                OuterStatement::Line(_, name) => panic!("Encountered line `{:?}` as an outer statement (7e35f035-1da2-41bc-a5eb-641699a16e93)", name),
            }
        }

        fn handle_inner_stmts(stmts: &Vec::<InnerStatement>, types: &mut HashMap<String, Type>, consts: &mut HashMap<String, yolol::ast::Expression>) -> Result<Vec<yolol::ast::Statement>, CompilerError> {
            Ok(stmts
                .iter()
                .map(|x| handle_inner_stmt(x, types, consts))
                .collect::<Result<Vec<_>, CompilerError>>()?
                .into_iter()
                .flatten()
                .collect()
            )
        }

        fn handle_inner_stmt(inner: &InnerStatement, types: &mut HashMap<String, Type>, consts: &mut HashMap<String, yolol::ast::Expression>) -> Result<Vec<yolol::ast::Statement>, CompilerError> {
            match inner {
                InnerStatement::CompilePanic(msg, pos) => Err(CompilerError::ExplicitPanic(msg.to_string(), *pos)),

                InnerStatement::Emit(code) => Err(CompilerError::CompilerStageNotImplemented(format!("Direct emit yolol code: `{}`", code))),

                InnerStatement::Call(name, args) => {
                    match find_call(name)? {
                        CallType::Macro => panic!("todo: macro call"),
                        CallType::Proc => panic!("todo: proc call"),
                    }
                },
                
                InnerStatement::If(condition, pass, fail) => {
                    Ok(vec![
                        yolol::ast::Statement::If(
                            handle_expr(condition, types)?,
                            Box::new(yolol::ast::StatementList { statements: handle_inner_stmts(pass, types, consts)? }),
                            Box::new(yolol::ast::StatementList { statements: handle_inner_stmts(fail, types, consts)? })
                        )
                    ])
                }

                InnerStatement::Assign(path, value) => {
                    
                    let name = canonicalise_field_path(&path);

                    // Type check expression and field type are compatible
                    match types.get(&name) {
                        Some(t) => type_check_assignment(&t, &infer_expr_type(&value, types)?),
                        None => Err(CompilerError::AssigningUndeclaredField(path.clone())),
                    }?;

                    let r = yolol::ast::Statement::Assignment(
                        yolol::ast::Identifier {
                            name: name,
                            external: false
                        },
                        handle_expr(value, types)?
                    );

                    return Ok(vec![r]);
                }

                InnerStatement::DeclareAssign(field, value) => {
                    if types.contains_key(&field.name) {
                        return Err(CompilerError::DuplicateFieldDeclaration(field.name.clone()));
                    }
    
                    type_check_assignment(&Type::Other(field.typename.clone()), &infer_expr_type(&value, types)?)?;
                    types.insert(field.name.clone(), field.typename.to_type());
    
                    let r = yolol::ast::Statement::Assignment(
                        yolol::ast::Identifier {
                            name: field.name.clone(),
                            external: false
                        },
                        handle_expr(value, types)?
                    );
    
                    return Ok(vec![r]);
                }

                InnerStatement::DeclareConst(field, value) => {
                    if types.contains_key(&field.name) {
                        return Err(CompilerError::DuplicateFieldDeclaration(field.name.clone()));
                    }

                    type_check_assignment(&Type::Other(field.typename.clone()), &infer_expr_type(&value, types)?)?;
                    types.insert(field.name.clone(), field.typename.to_type());
                    consts.insert(field.name.clone(), handle_expr(value, types)?);

                    return Ok(Vec::new());
                },

                InnerStatement::ExternalAssign(field, value) => {

                    //todo: type check externals?

                    let r = yolol::ast::Statement::Assignment(
                        yolol::ast::Identifier {
                            name: field.clone(),
                            external: true
                        },
                        handle_expr(value, types)?
                    );

                    return Ok(vec![r]);
                },

                InnerStatement::Return(value) => panic!("todo:Return"),
                
                InnerStatement::Goto(name) => {
                    Ok(vec![
                        yolol::ast::Statement::Goto(
                            yolol::ast::Expression::VariableAccess(
                                yolol::ast::Identifier {
                                    external: false,

                                    // todo: when lines are assigned in a later stage create this variable
                                    name: format!("goto_layout_label_{}", name)
                                }
                            )
                        )
                    ])
                }
            }
        }

        fn find_call(name: &str) -> Result<CallType, CompilerError> {
            return Err(CompilerError::CompilerStageNotImplemented(format!("Find Call `{}`", name)));
        }

        fn handle_expr(expr: &Expression, types: &mut HashMap<String, Type>) -> Result<yolol::ast::Expression, CompilerError> {
            Ok(match expr {

                Expression::CompilePanic(msg, pos) => return Err(CompilerError::ExplicitPanic(msg.to_string(), *pos)),

                Expression::ConstNumber(x) => yolol::ast::Expression::ConstantNumber(x.clone()),
                Expression::ConstString(x) => yolol::ast::Expression::ConstantString(x.clone()),

                Expression::Bracket(x) => yolol::ast::Expression::Bracket(Box::new(handle_expr(x, types)?)),
                Expression::Negate(x) => yolol::ast::Expression::Negate(Box::new(handle_expr(x, types)?)),
                Expression::Not(x) => yolol::ast::Expression::Not(Box::new(handle_expr(x, types)?)),

                Expression::Add(ref x, ref y) => yolol::ast::Expression::Add(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),
                Expression::Subtract(ref x, ref y) => yolol::ast::Expression::Subtract(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),
                Expression::Multiply(ref x, ref y) => yolol::ast::Expression::Multiply(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),
                Expression::Divide(ref x, ref y) => yolol::ast::Expression::Divide(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),
                Expression::Exponent(ref x, ref y) => yolol::ast::Expression::Exponent(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),
                Expression::Modulus(ref x, ref y) => yolol::ast::Expression::Modulus(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),
                Expression::And(ref x, ref y) => yolol::ast::Expression::And(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),
                Expression::Or(ref x, ref y) => yolol::ast::Expression::Or(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),

                Expression::GreaterThan(ref x, ref y) => yolol::ast::Expression::GreaterThan(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),
                Expression::GreaterThanOrEq(ref x, ref y) => yolol::ast::Expression::GreaterThanOrEq(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),
                Expression::LessThan(ref x, ref y) => yolol::ast::Expression::LessThan(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),
                Expression::LessThanOrEq(ref x, ref y) => yolol::ast::Expression::LessThanOrEq(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),
                Expression::Equals(ref x, ref y) => yolol::ast::Expression::Equal(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),
                Expression::NotEquals(ref x, ref y) => yolol::ast::Expression::NotEqual(Box::new(handle_expr(x, types)?), Box::new(handle_expr(y, types)?)),

                Expression::FieldAccess(x) => yolol::ast::Expression::VariableAccess(yolol::ast::Identifier { name: canonicalise_field_path(x), external: false }),
                Expression::ExternalFieldAccess(x) => yolol::ast::Expression::VariableAccess(yolol::ast::Identifier { name: x.clone(), external: true }),

                Expression::Call(name, args) => panic!("Call {:?} with {:?}", name, args),

                Expression::Is(ref expr, ref typename) => {
                    if type_check_assignment(&typename.to_type(), &infer_expr_type(expr, types)?).is_ok() {
                        yolol::ast::Expression::ConstantNumber(YololNumber::one())
                    } else {
                        yolol::ast::Expression::ConstantNumber(YololNumber::zero())
                    }
                },
                
                Expression::PostIncrement(name) => yolol::ast::Expression::PostDecrement(yolol::ast::Identifier { name: canonicalise_field_path(name), external: false }),
                Expression::PostDecrement(name) => yolol::ast::Expression::PostDecrement(yolol::ast::Identifier { name: canonicalise_field_path(name), external: false }),
                Expression::PreIncrement(name) => yolol::ast::Expression::PreIncrement(yolol::ast::Identifier { name: canonicalise_field_path(name), external: false }),
                Expression::PreDecrement(name) => yolol::ast::Expression::PreDecrement(yolol::ast::Identifier { name: canonicalise_field_path(name), external: false }),

                Expression::Constructor(typename, Initialisers) => panic!("Constructor for type {:?}", typename),
            })
        }
    }
}