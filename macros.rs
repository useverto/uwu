use crate::ast::{Expr, Ident, Literal};
pub enum Macro {
    Regexp,
}

impl Macro {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "regex" => Some(Macro::Regexp),
            _ => None,
        }
    }

    pub fn expand(&self, args: &Vec<Expr>) -> Option<String> {
        let mut source = String::new();
        match self {
            Macro::Regexp => {
                if args.len() > 0 {
                    source.push_str("/");
                    match &args[0] {
                        Expr::Ident(Ident(i)) => source.push_str(&i),
                        Expr::Literal(Literal::String(i)) => source.push_str(&i),
                        _ => return None,
                    }
                    source.push_str("/;");
                    return Some(source);
                }
                None
            }
        }
    }
}
