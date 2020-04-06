use std::collections::HashMap;

use crate::grammar::ast::{ Program, Main, InnerStatement, OuterStatement, CallableDefinition, StructDefinition };
use crate::error::{ CompilerError };
use super::super::build_config::BuildConfig;

#[derive(Debug)]
pub enum Block {
    // A set of statements that execute in the given order
    Statements(Option<String>, Vec<OuterStatement>),

    // A set of statements which must be put onto a single line of output
    Line(Option<String>, Vec<InnerStatement>)
}

#[derive(Debug)]
pub struct InitialStatementBlocks {
    pub blocks: Vec<Block>,

    pub callables: HashMap<String, CallableDefinition>,
    pub structs: HashMap<String, StructDefinition>,
}

impl Program {
    pub fn build_blocks(self, config: &BuildConfig) -> Result<InitialStatementBlocks, CompilerError> {

        fn extract_main(main: Main) -> Vec<Block> {
            let mut result: Vec<Block> = Vec::new();
            let mut current: Vec<OuterStatement> = Vec::new();
            let mut current_name = None;
    
            for stmt in main.statements.into_iter() {
                match stmt {
                    OuterStatement::Line(inner, label) => {
                        result.push(Block::Statements(current_name.clone(), current));
                        current = Vec::new();
                        current_name = None;
                        result.push(Block::Line(label, inner));
                    },
                    OuterStatement::Label(name) => {
                        result.push(Block::Statements(current_name.clone(), current));
                        current_name = Some(name.clone());
                        current = Vec::new();
                    }
                    stmt => current.push(stmt),
                }
            }
    
            result.push(Block::Statements(None, current));
    
            return result;
        }
    
        fn extract_calls(callables: Vec<CallableDefinition>) -> HashMap<String, CallableDefinition> {
            return callables.iter().map(|c| (c.name.clone(), c.clone())).collect();
        }

        return Ok(InitialStatementBlocks {
            blocks: extract_main(self.main.ok_or(CompilerError::NoMainBlock)?),
            callables: self.callables.iter().map(|c| (c.name.clone(), c.clone())).collect(),
            structs: self.structs.iter().map(|c| (c.name.clone(), c.clone())).collect(),
        });
    }
}