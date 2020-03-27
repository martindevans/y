use std::path::PathBuf;
use crate::compiler::Type;

pub enum CompilerError {
    IO(PathBuf, std::io::Error),
    Parse(PathBuf, String, peg_runtime::error::ParseError<peg_runtime::str::LineCol>),
    NoMainBlock,
    ExplicitPanic(String, usize),
    CompilerStageNotImplemented(String),
    DuplicateFieldDeclaration(String),
    AssigningUndeclaredField(Vec<String>),
    TypeCheckFailed(Type, Type),
}