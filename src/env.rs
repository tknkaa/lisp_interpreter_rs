use crate::types::Expr;
use std::collections::HashMap;

pub type Env = HashMap<String, Expr>;

pub fn default_env() -> Env {
    let env = HashMap::new();
    env
}
