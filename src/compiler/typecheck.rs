use crate::error::{ CompilerError };
use crate::grammar::ast::{ Expression };

#[derive(Debug, Clone)]
pub enum Type {
    Num,
    Str,
    Bool,
    Other(String)
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Num => write!(f, "number"),
            Type::Str => write!(f, "string"),
            Type::Bool => write!(f, "bool"),
            Type::Other(a) => write!(f, "{}", a),
        }
    }
}

pub fn expr_type(expr: &Expression) -> Result<Type, CompilerError> {
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

pub fn fixup_type_keywords(input: &Type) -> &Type {
    if let Type::Other(a) = input {
        if a == "number" { return &Type::Num }
        else if a == "string" { return &Type::Str }
        else if a == "bool" { return &Type::Bool }
        else { input }
    } else {
        return input;
    }
}

pub fn type_check(assign_to: &Type, assign_from: &Type) -> Result<(), CompilerError> {
    let assign_to = fixup_type_keywords(assign_to);
    let assign_from = fixup_type_keywords(assign_from);

    let err = Err(CompilerError::TypeCheckFailed(assign_to.clone(), assign_from.clone()));

    return match (assign_to, assign_from) {
        (Type::Bool, Type::Bool)     => Ok(()),
        (Type::Bool, Type::Num)      => err,
        (Type::Bool, Type::Str)      => err,
        (Type::Bool, Type::Other(_)) => err,

        (Type::Num, Type::Bool)      => Ok(()),
        (Type::Num, Type::Num)       => Ok(()),
        (Type::Num, Type::Str)       => err,
        (Type::Num, Type::Other(_))  => err,

        (Type::Other(_), Type::Bool)     => err,
        (Type::Other(_), Type::Num)      => err,
        (Type::Other(_), Type::Str)      => err,
        (Type::Other(a), Type::Other(b)) => if a == b { Ok(()) } else { err },

        (Type::Str, Type::Bool)      => err,
        (Type::Str, Type::Num)       => err,
        (Type::Str, Type::Str)       => Ok(()),
        (Type::Str, Type::Other(_))  => err,
    };
}