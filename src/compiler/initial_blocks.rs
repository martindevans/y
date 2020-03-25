use crate::grammar::ast::{ Program, InnerStatement, OuterStatement };
use crate::error::{ CompilerError };

#[derive(Debug)]
pub enum Block {
    // A set of statements that execute in the given order
    Statements(Option<String>, Vec<OuterStatement>),

    // A set of statements which must be put onto a single line of output
    Line(Option<String>, Vec<InnerStatement>)
}

#[derive(Debug)]
pub struct InitialStatementBlocks {
    pub blocks: Vec<Block>
}

impl Program {

    pub fn build_blocks(self) -> Result<InitialStatementBlocks, CompilerError> {
        
        let main = self.main.ok_or(CompilerError::NoMainBlock)?;

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

        return Ok(InitialStatementBlocks { blocks: result });
    }
}