use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Symbol(String),
    Number(i32),
    List(Vec<Expr>),
}

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
            Err(_) => Ok(Expr::Symbol(current.to_string())),
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

pub type Env = HashMap<String, Expr>;

// variables
pub fn default_env() -> Env {
    let env = HashMap::new();
    env
}

pub fn eval(expr: &Expr, env: &mut Env) -> Result<Expr, String> {
    match expr {
        Expr::Number(n) => Ok(Expr::Number(*n)),

        Expr::Symbol(s) => {
            let result = env.get(s);
            match result {
                Some(expr) => Ok(expr.clone()),
                None => Err(format!("undefined variable: {}", s)),
            }
        }

        Expr::List(items) => {
            if items.is_empty() {
                return Err("cannot eval empty list".to_string());
            }

            let func = &items[0];
            if let Expr::Symbol(name) = func {
                match name.as_str() {
                    "define" => {
                        let var_name = &items[1];
                        let value = eval(&items[2], env)?;
                        if let Expr::Symbol(var) = var_name {
                            env.insert(var.clone(), value.clone());
                            return Ok(value);
                        } else {
                            return Err("define requires a symbol as first argument".to_string());
                        }
                    }
                    _ => {}
                }
            }
            let args: Result<Vec<Expr>, String> =
                items[1..].iter().map(|arg| eval(arg, env)).collect();

            let args = args?;

            if let Expr::Symbol(func_name) = func {
                match func_name.as_str() {
                    "+" => {
                        let sum = args
                            .iter()
                            .map(|e| match e {
                                Expr::Number(n) => Ok(*n),
                                _ => Err("+ requires numbers".to_string()),
                            })
                            .collect::<Result<Vec<i32>, String>>()?
                            .iter()
                            .sum();
                        Ok(Expr::Number(sum))
                    }
                    "-" => {
                        let nums = args
                            .iter()
                            .map(|e| match e {
                                Expr::Number(n) => Ok(*n),
                                _ => Err("- requires numbers".to_string()),
                            })
                            .collect::<Result<Vec<i32>, String>>()?;
                        if nums.len() == 1 {
                            return Ok(Expr::Number((-1) * nums[0]));
                        }
                        let mut ans = nums[0] * 2;

                        for num in nums {
                            ans = ans - num;
                        }

                        return Ok(Expr::Number(ans));
                    }

                    "*" => {
                        let nums = args
                            .iter()
                            .map(|e| match e {
                                Expr::Number(n) => Ok(*n),
                                _ => Err("* requires numbers".to_string()),
                            })
                            .collect::<Result<Vec<i32>, String>>()?;
                        if nums.len() < 2 {
                            return Err("* requires at least 2 numbers".to_string());
                        }
                        let mut answer = 1;
                        for num in nums {
                            answer = answer * num;
                        }
                        return Ok(Expr::Number(answer));
                    }

                    "/" => {
                        let nums = args
                            .iter()
                            .map(|e| match e {
                                Expr::Number(n) => Ok(*n),
                                _ => Err("/ requires numbers".to_string()),
                            })
                            .collect::<Result<Vec<i32>, String>>()?;
                        let v0 = nums.get(0).ok_or("/ requires 2 numbers".to_string())?;

                        let v1 = nums.get(1).ok_or("/ requires 2 numbers".to_string())?;
                        if *v1 == 0 {
                            return Err("cannot divide by 0".to_string());
                        }
                        return Ok(Expr::Number(v0 / v1));
                    }
                    _ => Err(format!("unknown function: {}", func_name)),
                }
            } else {
                Err("first element must be a function name".to_string())
            }
        }
    }
}
