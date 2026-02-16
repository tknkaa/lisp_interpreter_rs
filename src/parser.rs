use crate::types::Expr;

pub fn parse(tokens: &Vec<&str>) -> Result<Expr, String> {
    if tokens.is_empty() {
        return Err("no tokens to parse".to_string());
    }
    let mut cursor = 0;
    let result = parse_value(tokens, &mut cursor)?;
    if cursor < tokens.len() {
        return Err(format!(
            "unexpected token '{}' after expression",
            tokens[cursor]
        ));
    }
    Ok(result)
}

fn parse_value(tokens: &Vec<&str>, cursor: &mut usize) -> Result<Expr, String> {
    let current = *tokens
        .get(*cursor)
        .ok_or("unexpected end of input".to_string())?;

    if current == "(" {
        parse_list(tokens, cursor)
    } else {
        *cursor += 1;
        match current.parse::<i32>() {
            Ok(num) => Ok(Expr::Number(num)),
            Err(_) => match current.parse::<bool>() {
                Ok(c) => Ok(Expr::Bool(c)),
                Err(_) => Ok(Expr::Symbol(current.to_string())),
            },
        }
    }
}

fn parse_list(tokens: &Vec<&str>, cursor: &mut usize) -> Result<Expr, String> {
    let mut list: Vec<Expr> = Vec::new();
    *cursor += 1;
    loop {
        match tokens.get(*cursor) {
            Some(v) => {
                if *v == ")" {
                    break;
                } else {
                    let value = parse_value(tokens, cursor)?;
                    list.push(value);
                }
            }
            None => {
                return Err("unclosed list: missing ')'".to_string());
            }
        }
    }
    *cursor += 1;
    Ok(Expr::List(list))
}
