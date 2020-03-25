use std::path::PathBuf;

pub enum CompilerError {
    IO(PathBuf, std::io::Error),
    Parse(PathBuf, String, peg_runtime::error::ParseError<peg_runtime::str::LineCol>),
    NoMainBlock,
    ExplicitPanic(String, usize),
    DuplicateFieldDeclaration(String),
    AssigningUndeclaredField(Vec<String>),
}