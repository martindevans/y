use std::collections::HashMap;

use crate::error::{ CompilerError };
use crate::grammar::ast::{ Expression, TypeName };
use crate::compiler::fields::{ canonicalise_field_path };

#[derive(Debug, Clone)]
pub enum Type {
    Any,
    Num,
    Str,
    Bool,
    Other(TypeName)
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Num => write!(f, "number"),
            Type::Str => write!(f, "string"),
            Type::Bool => write!(f, "bool"),
            Type::Other(a) => write!(f, "{:?}", a),
            Type::Any => write!(f, "any")
        }
    }
}

impl Type {
    pub fn to_typename(&self) -> TypeName {
        match self {
            Type::Any => TypeName { typename: "any".to_string() },
            Type::Num => TypeName { typename: "number".to_string() },
            Type::Bool => TypeName { typename: "bool".to_string() },
            Type::Str => TypeName { typename: "string".to_string() },
            Type::Other(a) => a.clone(),
        }
    }

    pub fn canonicalise(&self) -> Type {
        if let Type::Other(a) = self {
            if a.typename == "number" { return Type::Num }
            else if a.typename == "string" { return Type::Str }
            else if a.typename == "bool" { return Type::Bool }
            else if a.typename == "any" { return Type::Any }
            else { self.clone() }
        } else {
            return self.clone();
        }
    }
}

impl crate::grammar::ast::TypeName {
    pub fn to_type(&self) -> Type {
        let t = &Type::Other(self.clone());
        return t.canonicalise();
    }
}

pub fn infer_expr_type(expr: &Expression, fields: &HashMap<String, Type>) -> Result<Type, CompilerError> {

    let inference_failed = Err(CompilerError::ExpressionTypeInferenceFailed(expr.clone()));

    fn default_binary_expr(l: &Type, r: &Type, expr: &Expression, inference_failed: Result<Type, CompilerError>) -> Result<Type, CompilerError> {
        Ok(match (l, r) {
            (_, Type::Other(_)) => return inference_failed,
            (Type::Other(_), _) => return inference_failed,
            (Type::Any, _)      => return inference_failed,
            (_, Type::Any)      => return inference_failed,

            (Type::Num, Type::Num) => Type::Num,
            (Type::Num, Type::Bool) => Type::Num,
            (Type::Num, Type::Str) => Type::Str,

            (Type::Bool, Type::Num) => Type::Num,
            (Type::Bool, Type::Bool) => Type::Num,
            (Type::Bool, Type::Str) => Type::Str,

            (Type::Str, Type::Num) => Type::Str,
            (Type::Str, Type::Bool) => Type::Str,
            (Type::Str, Type::Str) => Type::Str
        })
    }

    Ok(match expr {
        Expression::ConstNumber(_) => Type::Num,
        Expression::ConstString(_) => Type::Str,

        Expression::Bracket(x) => infer_expr_type(x, fields)?,
        Expression::Not(_) => Type::Bool,

        Expression::And(_, _) => Type::Bool,
        Expression::Or(_, _) => Type::Bool,

        Expression::GreaterThan(_, _) => Type::Bool,
        Expression::GreaterThanOrEq(_, _) => Type::Bool,
        Expression::LessThan(_, _) => Type::Bool,
        Expression::LessThanOrEq(_, _) => Type::Bool,
        Expression::Equals(_, _) => Type::Bool,
        Expression::NotEquals(_, _) => Type::Bool,
        Expression::Is(_, _) => Type::Bool,

        Expression::Negate(a) => {
            let t = infer_expr_type(a, fields)?;
            match t {
                Type::Any => return inference_failed,
                Type::Num => Type::Num,
                Type::Bool => Type::Num,
                Type::Other(_) => return inference_failed,
                Type::Str => return Err(CompilerError::StaticTypeError("Negate a string".to_string(), expr.clone())),
            }
        }

        Expression::Add(a, b) => {
            let l = infer_expr_type(a, fields)?;
            let r = infer_expr_type(b, fields)?;
            default_binary_expr(&l, &r, expr, inference_failed)?
        }

        Expression::Subtract(a, b) => {
            let l = infer_expr_type(a, fields)?;
            let r = infer_expr_type(b, fields)?;
            default_binary_expr(&l, &r, expr, inference_failed)?
        }

        Expression::Multiply(a, b) => {
            let l = infer_expr_type(a, fields)?;
            let r = infer_expr_type(b, fields)?;
            match (l, r) {
                (Type::Num, Type::Str) => return Err(CompilerError::StaticTypeError("Multiply number by string".to_string(), expr.clone())),
                (Type::Bool, Type::Str) => return Err(CompilerError::StaticTypeError("Multiply bool by string".to_string(), expr.clone())),
                (Type::Str, Type::Num) => return Err(CompilerError::StaticTypeError("Multiply string by number".to_string(), expr.clone())),
                (Type::Str, Type::Bool) => return Err(CompilerError::StaticTypeError("Multiply string by bool".to_string(), expr.clone())),
                (Type::Str, Type::Str) => return Err(CompilerError::StaticTypeError("Multiply string by string".to_string(), expr.clone())),

                (l, r) => default_binary_expr(&l, &r, expr, inference_failed)?
            }
        }

        Expression::Divide(a, b) => {
            let l = infer_expr_type(a, fields)?;
            let r = infer_expr_type(b, fields)?;
            match (l, r) {
                (Type::Num, Type::Str) => return Err(CompilerError::StaticTypeError("Divide number by string".to_string(), expr.clone())),
                (Type::Bool, Type::Str) => return Err(CompilerError::StaticTypeError("Divide bool by string".to_string(), expr.clone())),
                (Type::Str, Type::Num) => return Err(CompilerError::StaticTypeError("Divide string by number".to_string(), expr.clone())),
                (Type::Str, Type::Bool) => return Err(CompilerError::StaticTypeError("Divide string by bool".to_string(), expr.clone())),
                (Type::Str, Type::Str) => return Err(CompilerError::StaticTypeError("Divide string by string".to_string(), expr.clone())),

                (l, r) => default_binary_expr(&l, &r, expr, inference_failed)?
            }
        }

        Expression::FieldAccess(f) => {
            let canonical = canonicalise_field_path(f);
            if let Some(t) = fields.get(&canonical) {
                return Ok(t.clone());
            } else {
                return Err(CompilerError::FieldTypeNotKnown(f.clone()));
            }
        },

        _ => return Err(CompilerError::CompilerStageNotImplemented(format!("Cannot infer type for expression: `{:?}`", expr)))
    })
}

pub fn type_check_assignment(assign_to: &Type, assign_from: &Type) -> Result<(), CompilerError> {
    let assign_to = assign_to.canonicalise();
    let assign_from = assign_from.canonicalise();

    let err = Err(CompilerError::TypeCheckFailed(assign_to.clone(), assign_from.clone()));

    return match (assign_to, assign_from) {
        (Type::Bool, Type::Bool)     => Ok(()),
        (Type::Bool, Type::Num)      => err,
        (Type::Bool, Type::Str)      => err,
        (Type::Bool, Type::Other(_)) => err,
        (Type::Bool, Type::Any)      => err,

        (Type::Num, Type::Bool)      => Ok(()),
        (Type::Num, Type::Num)       => Ok(()),
        (Type::Num, Type::Str)       => err,
        (Type::Num, Type::Other(_))  => err,
        (Type::Num, Type::Any)      => err,

        (Type::Other(_), Type::Bool)     => err,
        (Type::Other(_), Type::Num)      => err,
        (Type::Other(_), Type::Str)      => err,
        (Type::Other(a), Type::Other(b)) => if a == b { Ok(()) } else { err },
        (Type::Other(_), Type::Any)      => err,

        (Type::Str, Type::Bool)      => err,
        (Type::Str, Type::Num)       => err,
        (Type::Str, Type::Str)       => Ok(()),
        (Type::Str, Type::Other(_))  => err,
        (Type::Str, Type::Any)      => err,

        (Type::Any, Type::Bool) => Ok(()),
        (Type::Any, Type::Num) => Ok(()),
        (Type::Any, Type::Str) => Ok(()),
        (Type::Any, Type::Other(_)) => Ok(()),
        (Type::Any, Type::Any) => Ok(()),
    };
}