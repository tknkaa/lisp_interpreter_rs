use crate::env::Env;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Symbol(String),
    Number(i32),
    List(Vec<Expr>),
    Bool(bool),
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
        closure_env: Env,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Expr::Lambda { params, .. } => {
                write!(f, "<function({})>", params.join(", "))
            }
        }
    }
}
