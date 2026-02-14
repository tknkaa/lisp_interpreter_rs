pub fn tokenize(raw: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut buffer = String::new();
    for ch in raw.chars() {
        if ch == '(' || ch == ')' {
            if !buffer.is_empty() {
                tokens.push(buffer.clone());
                buffer.clear();
            }
            tokens.push(ch.to_string());
        } else if ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n' {
            if !buffer.is_empty() {
                tokens.push(buffer.clone());
                buffer.clear();
            }
        } else {
            buffer.push(ch);
        }
    }
    if !buffer.is_empty() {
        tokens.push(buffer.clone());
    }
    tokens
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Symbol(String),
    Number(u32),
    List(Vec<Expr>),
}

pub fn parse(tokens: &Vec<&str>) -> Result<Expr, ()> {
    Ok(Expr::Number(1))
}
