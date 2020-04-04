use std::collections::HashMap;

use uuid::Uuid;

use crate::error::{ CompilerError };
use crate::grammar::ast::{ InnerStatement, OuterStatement, Expression, CallableDefinition, FieldDefinition, CallType, ParameterDefinition };
use super::initial_blocks::{ InitialStatementBlocks, Block };
use super::super::typecheck::{ infer_expr_type, Type, type_check_assignment };
use super::super::build_config::BuildConfig;

impl InitialStatementBlocks {
    pub fn inline_macros(self, config: &BuildConfig) -> Result<InitialStatementBlocks, CompilerError> {

        let blocks = self.blocks;
        let callables = self.callables;
        let mut types = HashMap::new();

        let result: Result<_, _> = blocks
            .into_iter()
            .map(|x| handle_block(x, &callables, &mut types, config))
            .collect();

        return Ok(InitialStatementBlocks {
            blocks: result?,
            callables: callables,
        });

        fn handle_block(b: Block, callables: &HashMap<String, CallableDefinition>, types: &mut HashMap<String, Type>, config: &BuildConfig) -> Result<Block, CompilerError> {
            match b {
                Block::Statements(label, stmts) => Ok(Block::Statements(label, handle_outer_stmts(stmts, callables, types, config)?)),
                Block::Line(label, stmts) => Ok(Block::Line(label, handle_inner_stmts(stmts, callables, types, config)?)),
            }
        }

        fn handle_outer_stmts(stmts: Vec::<OuterStatement>, callables: &HashMap<String, CallableDefinition>, types: &mut HashMap<String, Type>, config: &BuildConfig) -> Result<Vec<OuterStatement>, CompilerError> {
            Ok(stmts
                .into_iter()
                .map(|x| handle_outer_stmt(x, callables, types, config))
                .collect::<Result<Vec<_>, CompilerError>>()?
                .into_iter()
                .flatten()
                .collect()
            )
        }

        fn handle_outer_stmt(outer: OuterStatement, callables: &HashMap<String, CallableDefinition>, types: &mut HashMap<String, Type>, config: &BuildConfig) -> Result<Vec<OuterStatement>, CompilerError> {
            match outer {
                OuterStatement::Inner(inner) => Ok(handle_inner_stmt(inner, callables, types, config)?.into_iter().map(|x| OuterStatement::Inner(x)).collect()),

                // There should be no `Label` statements here, they've been promoted into named blocks by the previous pass
                OuterStatement::Label(name) => panic!("Encountered label `{}` as an outer statement (1b566df9-0d58-4bd8-aade-89a2e5a4397b)", name),

                // There should be no `Line` statements here, they've been separated into Line blocks by the previous pass
                OuterStatement::Line(_, name) => panic!("Encountered line `{:?}` as an outer statement (bf0389c9-db37-4019-99e9-62747d842146)", name),
            }
        }

        fn handle_inner_stmts(stmts: Vec::<InnerStatement>, callables: &HashMap<String, CallableDefinition>, types: &mut HashMap<String, Type>, config: &BuildConfig) -> Result<Vec<InnerStatement>, CompilerError> {
            Ok(stmts
                .into_iter()
                .map(|x| handle_inner_stmt(x, callables, types, config))
                .collect::<Result<Vec<_>, CompilerError>>()?
                .into_iter()
                .flatten()
                .collect()
            )
        }

        fn handle_inner_stmt(inner: InnerStatement, callables: &HashMap<String, CallableDefinition>, types: &mut HashMap<String, Type>, config: &BuildConfig) -> Result<Vec<InnerStatement>, CompilerError> {
            match inner {
                InnerStatement::Call(ref name, ref args) => handle_call_stmt(name, args, callables, None, types, config),

                InnerStatement::DeclareAssign(field, value) => {
                    if types.contains_key(&field.name) {
                        return Err(CompilerError::DuplicateFieldDeclaration(field.name.clone()));
                    }

                    types.insert(field.name.clone(), field.typename.to_type());
    
                    return Ok(vec![InnerStatement::DeclareAssign(field, value)]);
                }

                InnerStatement::DeclareConst(field, value) => {
                    if types.contains_key(&field.name) {
                        return Err(CompilerError::DuplicateFieldDeclaration(field.name.clone()));
                    }

                    types.insert(field.name.clone(), field.typename.to_type());

                    return Ok(vec![InnerStatement::DeclareConst(field, value)]);
                },

                //todo: check expressions for embedded calls
                a => Ok(vec![a])
            }
        }

        fn handle_call_stmt(name: &String, args: &Vec<Expression>, callables: &HashMap<String, CallableDefinition>, return_field: Option<Vec<String>>, types: &HashMap<String, Type>, config: &BuildConfig) -> Result<Vec<InnerStatement>, CompilerError> {

            let callable = match callables.get(name) {
                Some(c) => c,
                None => return Err(CompilerError::CallableNotFound(name.clone()))
            };

            if callable.return_type.is_some() {
                return Err(CompilerError::CompilerStageNotImplemented("return values from calls are not implemented".to_string()));
            }

            if callable.attributes.len() != 0 {
                return Err(CompilerError::CompilerStageNotImplemented("Call attributes are not implemented".to_string()));
            }

            if let CallType::Proc = callable.call_type {
                return Err(CompilerError::CompilerStageNotImplemented("`proc` calls are not implemented".to_string()));
            }

            // Check that the correct numbers of parameters were passed
            if callable.parameters.len() != args.len() {
                return Err(CompilerError::IncorrectCallParameterCount(name.clone(), callable.parameters.len(), args.len()));
            }

            let mut result: Vec<InnerStatement> = Vec::new();
            let mut bindings: HashMap<&ParameterDefinition, Expression> = HashMap::new();
            
            // Build a list of bindings, every time a parameter is accessed inside the macro body the binding value will be used instead
            for (param, arg) in callable.parameters.iter().zip(args) {
                match arg {
                    Expression::ExternalFieldAccess(name) => { bindings.insert(param, Expression::ExternalFieldAccess(name.clone())); }, //todo: type check externals?
                    Expression::FieldAccess(name) => { bindings.insert(param, Expression::FieldAccess(name.clone())); },
                    Expression::ConstNumber(num) => { bindings.insert(param, Expression::ConstNumber(num.clone())); },
                    Expression::ConstString(string) => { bindings.insert(param, Expression::ConstString(string.clone())); },
                    other => {
                        // Assign expression value to a temp, pass temp into macro
                        let n = format!("_tmp_{}", Uuid::new_v4().to_simple());
                        let t = infer_expr_type(other, types)?;
                        let d = OuterStatement::Inner(InnerStatement::DeclareAssign(
                            FieldDefinition { name: n.clone(), typename: infer_expr_type(other, types)?.to_typename() },
                            other.clone()
                        ));
                        bindings.insert(param, Expression::FieldAccess(vec![n]));
                    }
                };
            }

            // Check that every binding has a compatible type with the parameter it's bound to
            for (param, expr) in bindings.iter() {
                type_check_assignment(&param.field.typename.to_type(), &infer_expr_type(&expr, &types)?)?;
            }

            // Rewrite the macro AST to replace field accesses. If they're an access to a parameter, use the expression from the `bindings` map
            // otherwise mangle the name to a temporary which is only valid inside this call body.
            // todo: ^
            let mut rewritten = callable.clone();
            panic!("rewrite");

            // Now emit the rewritten AST
            result.append(&mut rewritten.statements);

            return Ok(result);
        }
    }
}