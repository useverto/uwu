use crate::ast::{Expr, Literal};

#[derive(Debug)]
pub enum Type {
    String,
    Number,
    Array(Vec<Type>),
    Bool,
    Hash(Vec<(Type, Type)>),
    Function,
    Unknown,
}

#[macro_export]
macro_rules! ltype {
    ($e: expr) => {
        $crate::types::Type::from_literal($e)
    };
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number, Self::Number) => true,
            (Self::Bool, Self::Bool) => true,
            (Self::String, Self::String) => true,
            (Self::Array(_), Self::Array(_)) => true,
            (Self::Hash(_), Self::Hash(_)) => true,
            (Self::Function, Self::Function) => true,
            (Self::Unknown, _) => true,
            (_, Self::Unknown) => true,
            _ => false,
        }
    }
}

impl Type {
    pub fn from_literal(lit: &Literal) -> Self {
        match lit {
            Literal::Number(_) => Type::Number,
            Literal::String(_) => Type::String,
            Literal::Array(v) => {
                let mut t: Vec<Type> = vec![];
                for expr in v {
                    match expr {
                        Expr::Literal(l) => t.push(Self::from_literal(l)),
                        _ => t.push(Self::Unknown),
                    }
                }
                Self::Array(t)
            }
            Literal::Bool(_) => Type::Bool,
            Literal::Hash(o) => {
                let mut t: Vec<(Self, Self)> = vec![];
                for (k, v) in o {
                    let kt = match k {
                        Expr::Literal(l) => Self::from_literal(l),
                        _ => Self::Unknown,
                    };
                    let vt = match v {
                        Expr::Literal(l) => Self::from_literal(l),
                        _ => Self::Unknown,
                    };
                    t.push((kt, vt));
                }
                Self::Hash(t)
            }
        }
    }
}
