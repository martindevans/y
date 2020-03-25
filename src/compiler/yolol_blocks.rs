use std::collections::HashMap;
use std::collections::HashSet;

use rayon::prelude::*;

use crate::grammar::ast::{ InnerStatement, OuterStatement, TypeName, Expression, FieldDefinition };
use crate::error::{ CompilerError };
use crate::yolol;

use super::initial_blocks::{ InitialStatementBlocks, Block };

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
                OuterStatement::Label(name) => panic!("Encountered a label as an outer statement (6fe60057-c45d-4a2b-882d-308cd141d4e1)"),

                // There should be no `Line` statements here, they've been separated into Line blocks by the previous pass
                OuterStatement::Line(name, stmts) => panic!("Encountered a line block as an outer statement (7e35f035-1da2-41bc-a5eb-641699a16e93)"),
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

                InnerStatement::Emit(code) => panic!("todo: parse yolol and emit verbatim"),

                InnerStatement::Call(name, args) => panic!("todo: call"),
                
                InnerStatement::If(condition, pass, fail) => panic!("todo:if"),

                InnerStatement::Assign(path, value) => {
                    
                    let name = convert_field_path(&path);

                    // Type check expression and field type are compatible
                    match types.get(&name) {
                        Some(t) => type_check(t, &expr_type(&value)),
                        None => Err(CompilerError::AssigningUndeclaredField(path.clone())),
                    }?;

                    let r = yolol::ast::Statement::Assignment(
                        yolol::ast::Identifier {
                            name: name,
                            external: false
                        },
                        handle_expr(value)
                    );

                    return Ok(vec![r]);
                }

                InnerStatement::DeclareAssign(field, value) => {
                    if types.contains_key(&field.name) {
                        return Err(CompilerError::DuplicateFieldDeclaration(field.name.clone()));
                    }
    
                    type_check(&field.typename, &expr_type(&value))?;
                    types.insert(field.name.clone(), field.typename.clone());
    
                    let r = yolol::ast::Statement::Assignment(
                        yolol::ast::Identifier {
                            name: field.name.clone(),
                            external: false
                        },
                        handle_expr(value)
                    );
    
                    return Ok(vec![r]);
                }

                InnerStatement::DeclareConst(field, value) => {
                    if types.contains_key(&field.name) {
                        return Err(CompilerError::DuplicateFieldDeclaration(field.name.clone()));
                    }

                    type_check(&field.typename, &expr_type(&value))?;
                    types.insert(field.name.clone(), field.typename.clone());
                    consts.insert(field.name.clone(), handle_expr(value));

                    return Ok(Vec::new());
                },

                InnerStatement::ExternalAssign(field, value) => {

                    //todo: type check externals?

                    let r = yolol::ast::Statement::Assignment(
                        yolol::ast::Identifier {
                            name: field.clone(),
                            external: true
                        },
                        handle_expr(value)
                    );

                    return Ok(vec![r]);
                },

                InnerStatement::Return(value) => panic!("todo:Return"),
                InnerStatement::Goto(name) => panic!("todo:Goto"),
            }
        }

        fn handle_expr(expr: &Expression) -> yolol::ast::Expression {

            match expr {
                Expression::ConstNumber(x) => yolol::ast::Expression::ConstantNumber(x.clone()),
                _ => panic!(format!("todo expr: `{:?}`", expr))
            }
        }

        fn convert_field_path(path: &Vec<String>) -> String {
            return path.join("_");
        }

        fn expr_type(expr: &Expression) -> TypeName {
            return TypeName { typename:"todo".to_string() };
        }

        fn type_check(assign_to: &TypeName, assign_from: &TypeName) -> Result<(), CompilerError> {
            return Ok(()); //todo: type check
        }
    }
}