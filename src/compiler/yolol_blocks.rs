use std::collections::HashMap;

use rayon::prelude::*;

use crate::grammar::ast::{ InnerStatement, OuterStatement, TypeName, Expression, FieldDefinition };
use crate::error::{ CompilerError };
use crate::yolol;

use super::initial_blocks::{ InitialStatementBlocks, Block };

#[derive(Debug)]
enum CallType {
    Macro,
    Proc,
}

#[derive(Debug)]
enum Type {
    Num,
    Str,
    Bool,
    Other(String)
}

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

    pub types: HashMap<String, TypeName>,
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

        fn handle_block(b: &Block, types: &mut HashMap<String, TypeName>, consts: &mut HashMap<String, yolol::ast::Expression>) -> Result<YololBlock, CompilerError> {
            match b {
                Block::Statements(name, stmts) => Ok(YololBlock::Statements(name.clone(), handle_outer_stmts(stmts, types, consts)?)),
                Block::Line(label, stmts) => Ok(YololBlock::Line(label.clone(), handle_inner_stmts(stmts, types, consts)?))
            }
        }

        fn handle_outer_stmts(stmts: &Vec::<OuterStatement>, types: &mut HashMap<String, TypeName>, consts: &mut HashMap<String, yolol::ast::Expression>) -> Result<Vec<yolol::ast::Statement>, CompilerError> {
            Ok(stmts
                .iter()
                .map(|x| handle_outer_stmt(x, types, consts))
                .collect::<Result<Vec<_>, CompilerError>>()?
                .into_iter()
                .flatten()
                .collect()
            )
        }

        fn handle_outer_stmt(inner: &OuterStatement, types: &mut HashMap<String, TypeName>, consts: &mut HashMap<String, yolol::ast::Expression>) -> Result<Vec<yolol::ast::Statement>, CompilerError> {
            match inner {
                OuterStatement::Inner(inner) => handle_inner_stmt(&inner, types, consts),

                // There should be no `Label` statements here, they've been promoted into named blocks by the previous pass
                OuterStatement::Label(name) => panic!("Encountered label `{}` as an outer statement (6fe60057-c45d-4a2b-882d-308cd141d4e1)", name),

                // There should be no `Line` statements here, they've been separated into Line blocks by the previous pass
                OuterStatement::Line(_, name) => panic!("Encountered line `{:?}` as an outer statement (7e35f035-1da2-41bc-a5eb-641699a16e93)", name),
            }
        }

        fn handle_inner_stmts(stmts: &Vec::<InnerStatement>, types: &mut HashMap<String, TypeName>, consts: &mut HashMap<String, yolol::ast::Expression>) -> Result<Vec<yolol::ast::Statement>, CompilerError> {
            Ok(stmts
                .iter()
                .map(|x| handle_inner_stmt(x, types, consts))
                .collect::<Result<Vec<_>, CompilerError>>()?
                .into_iter()
                .flatten()
                .collect()
            )
        }

        fn handle_inner_stmt(inner: &InnerStatement, types: &mut HashMap<String, TypeName>, consts: &mut HashMap<String, yolol::ast::Expression>) -> Result<Vec<yolol::ast::Statement>, CompilerError> {
            match inner {
                InnerStatement::CompilePanic(msg, pos) => Err(CompilerError::ExplicitPanic(msg.to_string(), *pos)),

                InnerStatement::Emit(code) => Err(CompilerError::CompilerStageNotImplemented(format!("Direct emit yolol code: `{}`", code))),

                InnerStatement::Call(name, args) => {
                    match find_call(name) {
                        Macro => panic!("todo: macro call"),
                        Call => panic!("todo: proc call"),
                    }
                },
                
                InnerStatement::If(condition, pass, fail) => {
                    Ok(vec![
                        yolol::ast::Statement::If(
                            handle_expr(condition)?,
                            Box::new(yolol::ast::StatementList { statements: handle_inner_stmts(pass, types, consts)? }),
                            Box::new(yolol::ast::StatementList { statements: handle_inner_stmts(fail, types, consts)? })
                        )
                    ])
                }

                InnerStatement::Assign(path, value) => {
                    
                    let name = convert_field_path(&path);

                    // Type check expression and field type are compatible
                    match types.get(&name) {
                        Some(t) => type_check(&Type::Other(t.typename.clone()), &expr_type(&value)?),
                        None => Err(CompilerError::AssigningUndeclaredField(path.clone())),
                    }?;

                    let r = yolol::ast::Statement::Assignment(
                        yolol::ast::Identifier {
                            name: name,
                            external: false
                        },
                        handle_expr(value)?
                    );

                    return Ok(vec![r]);
                }

                InnerStatement::DeclareAssign(field, value) => {
                    if types.contains_key(&field.name) {
                        return Err(CompilerError::DuplicateFieldDeclaration(field.name.clone()));
                    }
    
                    type_check(&Type::Other(field.typename.typename.clone()), &expr_type(&value)?)?;
                    types.insert(field.name.clone(), field.typename.clone());
    
                    let r = yolol::ast::Statement::Assignment(
                        yolol::ast::Identifier {
                            name: field.name.clone(),
                            external: false
                        },
                        handle_expr(value)?
                    );
    
                    return Ok(vec![r]);
                }

                InnerStatement::DeclareConst(field, value) => {
                    if types.contains_key(&field.name) {
                        return Err(CompilerError::DuplicateFieldDeclaration(field.name.clone()));
                    }

                    type_check(&Type::Other(field.typename.typename.clone()), &expr_type(&value)?)?;
                    types.insert(field.name.clone(), field.typename.clone());
                    consts.insert(field.name.clone(), handle_expr(value)?);

                    return Ok(Vec::new());
                },

                InnerStatement::ExternalAssign(field, value) => {

                    //todo: type check externals?

                    let r = yolol::ast::Statement::Assignment(
                        yolol::ast::Identifier {
                            name: field.clone(),
                            external: true
                        },
                        handle_expr(value)?
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

        fn find_call(name: &str) -> CallType {
            panic!("find call");
        }

        fn handle_expr(expr: &Expression) -> Result<yolol::ast::Expression, CompilerError> {
            Ok(match expr {
                Expression::ConstNumber(x) => yolol::ast::Expression::ConstantNumber(x.clone()),
                Expression::ConstString(x) => yolol::ast::Expression::ConstantString(x.clone()),

                Expression::Bracket(x) => yolol::ast::Expression::Bracket(Box::new(handle_expr(x)?)),
                Expression::Negate(x) => yolol::ast::Expression::Negate(Box::new(handle_expr(x)?)),
                Expression::Not(x) => yolol::ast::Expression::Not(Box::new(handle_expr(x)?)),

                Expression::Add(x, y) => yolol::ast::Expression::Add(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),
                Expression::Subtract(x, y) => yolol::ast::Expression::Subtract(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),
                Expression::Multiply(x, y) => yolol::ast::Expression::Multiply(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),
                Expression::Divide(x, y) => yolol::ast::Expression::Divide(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),
                Expression::Exponent(x, y) => yolol::ast::Expression::Exponent(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),
                Expression::Modulus(x, y) => yolol::ast::Expression::Modulus(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),
                Expression::And(x, y) => yolol::ast::Expression::And(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),
                Expression::Or(x, y) => yolol::ast::Expression::Or(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),

                Expression::GreaterThan(x, y) => yolol::ast::Expression::GreaterThan(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),
                Expression::GreaterThanOrEq(x, y) => yolol::ast::Expression::GreaterThanOrEq(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),
                Expression::LessThan(x, y) => yolol::ast::Expression::LessThan(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),
                Expression::LessThanOrEq(x, y) => yolol::ast::Expression::LessThanOrEq(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),
                Expression::Equals(x, y) => yolol::ast::Expression::Equal(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),
                Expression::NotEquals(x, y) => yolol::ast::Expression::NotEqual(Box::new(handle_expr(&x)?), Box::new(handle_expr(&y)?)),

                _ => return Err(CompilerError::CompilerStageNotImplemented(format!("Unhandled expr: `{:?}`", expr)))
            })
        }
        

        fn convert_field_path(path: &Vec<String>) -> String {
            return path.join("_");
        }

        fn expr_type(expr: &Expression) -> Result<Type, CompilerError> {
            Ok(match expr {

                Expression::ConstNumber(_) => Type::Num,
                Expression::ConstString(_) => Type::Str,

                Expression::Bracket(x) => expr_type(x)?,
                Expression::Not(_) => Type::Bool,

                Expression::And(_, _) => Type::Bool,
                Expression::Or(_, _) => Type::Bool,

                Expression::GreaterThan(_, _) => Type::Bool,
                Expression::GreaterThanOrEq(_, _) => Type::Bool,
                Expression::LessThan(_, _) => Type::Bool,
                Expression::LessThanOrEq(_, _) => Type::Bool,
                Expression::Equals(_, _) => Type::Bool,
                Expression::NotEquals(_, _) => Type::Bool,

                _ => return Err(CompilerError::CompilerStageNotImplemented(format!("Unhandled expr: `{:?}`", expr)))
            })
        }

        fn type_check(assign_to: &Type, assign_from: &Type) -> Result<(), CompilerError> {
            return Err(CompilerError::CompilerStageNotImplemented("typecheck".to_string()));
        }
    }
}