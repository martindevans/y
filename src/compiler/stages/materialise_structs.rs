use std::collections::HashMap;

use crate::error::{ CompilerError };
use crate::grammar::ast::{ InnerStatement, OuterStatement, Expression, StructDefinition, FieldDefinition, CallType, ParameterDefinition };
use super::initial_blocks::{ InitialStatementBlocks, Block };
use super::super::typecheck::{ infer_expr_type, Type, type_check_assignment };
use super::super::build_config::BuildConfig;
use super::super::fields::{ canonicalise_field_path };

impl InitialStatementBlocks {

    pub fn materialise_structs(self, config: &BuildConfig) -> Result<InitialStatementBlocks, CompilerError> {

        // todo: Replace initialising struct with individual fields from that struct
        // todo: Replace all uses of a struct with individual fields from that struct

        let structs = self.structs;

        let mut types = HashMap::new();
        let result: Result<_, _> = self.blocks
            .into_iter()
            .map(|x| handle_block(x, &structs, &mut types, config))
            .collect();

        return Ok(InitialStatementBlocks {
            blocks: result?,
            callables: self.callables,
            structs: structs,
        });

        fn handle_block(b: Block, structs: &HashMap<String, StructDefinition>, types: &mut HashMap<String, Type>, config: &BuildConfig) -> Result<Block, CompilerError> {
            match b {
                Block::Statements(label, stmts) => Ok(Block::Statements(label, handle_outer_stmts(stmts, structs, types, config)?)),
                Block::Line(label, stmts) => Ok(Block::Line(label, handle_inner_stmts(stmts, structs, types, config)?)),
            }
        }

        fn handle_outer_stmts(stmts: Vec::<OuterStatement>, structs: &HashMap<String, StructDefinition>, types: &mut HashMap<String, Type>, config: &BuildConfig) -> Result<Vec<OuterStatement>, CompilerError> {
            Ok(stmts
                .into_iter()
                .map(|x| handle_outer_stmt(x, structs, types, config))
                .collect::<Result<Vec<_>, CompilerError>>()?
                .into_iter()
                .flatten()
                .collect()
            )
        }

        fn handle_outer_stmt(outer: OuterStatement, structs: &HashMap<String, StructDefinition>, types: &mut HashMap<String, Type>, config: &BuildConfig) -> Result<Vec<OuterStatement>, CompilerError> {
            match outer {
                OuterStatement::Inner(inner) => Ok(handle_inner_stmt(inner, structs, types, config)?.into_iter().map(|x| OuterStatement::Inner(x)).collect()),

                // There should be no `Label` statements here, they've been promoted into named blocks by the initial_blocks pass
                OuterStatement::Label(name) => panic!("Encountered label `{}` as an outer statement (13383ff8-d242-40d2-936f-afc26199e016)", name),

                // There should be no `Line` statements here, they've been separated into Line blocks by the initial_blocks pass
                OuterStatement::Line(_, name) => panic!("Encountered line `{:?}` as an outer statement (0dc74572-9c3d-4a64-be5f-1293d26c7fb8)", name),
            }
        }

        fn handle_inner_stmts(stmts: Vec::<InnerStatement>, structs: &HashMap<String, StructDefinition>, types: &mut HashMap<String, Type>, config: &BuildConfig) -> Result<Vec<InnerStatement>, CompilerError> {
            Ok(stmts
                .into_iter()
                .map(|x| handle_inner_stmt(x, structs, types, config))
                .collect::<Result<Vec<_>, CompilerError>>()?
                .into_iter()
                .flatten()
                .collect()
            )
        }

        fn handle_inner_stmt(inner: InnerStatement, structs: &HashMap<String, StructDefinition>, types: &mut HashMap<String, Type>, config: &BuildConfig) -> Result<Vec<InnerStatement>, CompilerError> {

            fn handle_constructor(field: &String, typ: &Type, structs: &HashMap<String, StructDefinition>, args: &Vec<(String, Expression)>) -> Result<Vec<InnerStatement>, CompilerError> {

                let typ = match typ {
                    Type::Other(a) => a,
                    _ => return Err(CompilerError::FieldConstructorAssignment(typ.clone(), args.clone())),
                };

                panic!("check that constructor args are compatible with type");
                panic!("constructor declaration initialiser");
            }

            match inner {

                InnerStatement::DeclareAssign(field, Expression::Constructor(args)) => handle_constructor(&field.name, &field.typename.to_type(), &structs, &args),

                InnerStatement::Assign(field, Expression::Constructor(args)) => {
                    let n = &canonicalise_field_path(&field);
                    let t = types.get(n);
                    match t {
                        None => return Err(CompilerError::FieldTypeNotKnown(field.clone())),
                        Some(t) => handle_constructor(n, t, &structs, &args)
                    }
                }

                other => Ok(vec![other]),
            }
        }
    }

}