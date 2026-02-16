use crate::env::Env;
use crate::types::Expr;

pub fn eval(expr: &Expr, env: &mut Env) -> Result<Expr, String> {
    match expr {
        Expr::Number(n) => Ok(Expr::Number(*n)),
        Expr::Bool(b) => Ok(Expr::Bool(*b)),
        Expr::Symbol(s) => {
            let result = env.get(s);
            match result {
                Some(expr) => Ok(expr.clone()),
                None => Err(format!("undefined variable: {}", s)),
            }
        }
        Expr::Lambda {
            params,
            body,
            closure_env,
        } => Ok(Expr::Lambda {
            params: params.clone(),
            body: body.clone(),
            closure_env: closure_env.clone(),
        }),

        Expr::List(items) => {
            if items.is_empty() {
                return Err("cannot eval empty list".to_string());
            }

            let func = &items[0];
            // Special forms: These control when/if their arguments are evaluated
            // They must be handled BEFORE evaluating arguments (unlike regular functions)
            if let Expr::Symbol(name) = func {
                match name.as_str() {
                    "define" => {
                        // Special form: (define x 10)
                        // - x must NOT be evaluated (stays as symbol)
                        // - only the value (10) is evaluated
                        let var_name = &items[1];
                        let value = eval(&items[2], env)?;
                        if let Expr::Symbol(var) = var_name {
                            env.insert(var.clone(), value.clone());
                            return Ok(value);
                        } else {
                            return Err("define requires a symbol as first argument".to_string());
                        }
                    }
                    "if" => {
                        // Special form: (if condition then-branch else-branch)
                        // - Only evaluates the condition first
                        // - Then evaluates ONLY ONE branch (not both)
                        // - Example: (if false 1 (/ 1 0)) won't error because (/ 1 0) never runs
                        if items.len() != 4 {
                            return Err("if requires 3 arguments".to_string());
                        }
                        let condition = eval(&items[1], env)?;
                        let cond_bool = match condition {
                            Expr::Bool(b) => b,
                            Expr::Number(n) => n != 0,
                            _ => return Err("if condition must be a boolean".to_string()),
                        };
                        if cond_bool {
                            return eval(&items[2], env);
                        } else {
                            return eval(&items[3], env);
                        }
                    }
                    "lambda" => {
                        // Special form: (lambda (x y) (+ x y))
                        // - Parameters are NOT evaluated (stay as symbols)
                        // - Body is NOT evaluated yet (evaluated when function is called)
                        // - Captures the current environment (closure)
                        if items.len() != 3 {
                            return Err("lambda requires 2 arguments: params and body".to_string());
                        }

                        let params = match &items[1] {
                            Expr::List(param_list) => {
                                let mut params = Vec::new();
                                for param in param_list {
                                    if let Expr::Symbol(name) = param {
                                        params.push(name.clone());
                                    } else {
                                        return Err("lambda parameters must be symbols".to_string());
                                    }
                                }
                                params
                            }
                            _ => return Err("lambda parameters must be a list".to_string()),
                        };

                        let body = Box::new(items[2].clone());
                        let closure_env = env.clone();

                        return Ok(Expr::Lambda {
                            params,
                            body,
                            closure_env,
                        });
                    }
                    _ => {}
                }
            }
            // Regular functions: ALL arguments are evaluated first, then passed to the function
            // This is done here (line below) BEFORE matching function names
            // Example: (+ 1 (+ 2 3)) → evaluates 1 and (+ 2 3) first → (+ 1 5) → 6
            let args: Result<Vec<Expr>, String> =
                items[1..].iter().map(|arg| eval(arg, env)).collect();

            let args = args?;

            // Check if func is a Symbol (built-in operator or variable holding a lambda)
            // or if it needs evaluation (e.g., nested lambda call)
            let func_evaled = if let Expr::Symbol(name) = func {
                // Try to get from environment (might be a user-defined function)
                if let Some(val) = env.get(name) {
                    val.clone()
                } else {
                    // It's a built-in function, keep as symbol
                    func.clone()
                }
            } else {
                // Not a symbol, evaluate it (e.g., ((lambda ...) args))
                eval(func, env)?
            };

            // Check if it's a user-defined function (lambda)
            if let Expr::Lambda {
                params,
                body,
                closure_env,
            } = func_evaled
            {
                if params.len() != args.len() {
                    return Err(format!(
                        "function expects {} arguments, got {}",
                        params.len(),
                        args.len()
                    ));
                }

                // Create new environment: start with current env (for recursion),
                // then add closure env (for lexical scoping), then params
                let mut new_env = env.clone();
                for (k, v) in closure_env.iter() {
                    new_env.insert(k.clone(), v.clone());
                }

                // Bind parameters to arguments
                for (param, arg) in params.iter().zip(args.iter()) {
                    new_env.insert(param.clone(), arg.clone());
                }

                // Evaluate body in the new environment
                return eval(&body, &mut new_env);
            }

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
                        let answer = nums.iter().fold(1, |acc, v| acc * v);
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

                    ">" => {
                        if args.len() != 2 {
                            return Err("> requires exactly 2 arguments".to_string());
                        }
                        let nums = args
                            .iter()
                            .map(|e| match e {
                                Expr::Number(n) => Ok(*n),
                                _ => Err("> requires numbers".to_string()),
                            })
                            .collect::<Result<Vec<i32>, String>>()?;
                        return Ok(Expr::Bool(nums[0] > nums[1]));
                    }

                    "<" => {
                        if args.len() != 2 {
                            return Err("< requires exactly 2 arguments".to_string());
                        }
                        let nums = args
                            .iter()
                            .map(|e| match e {
                                Expr::Number(n) => Ok(*n),
                                _ => Err("< requires numbers".to_string()),
                            })
                            .collect::<Result<Vec<i32>, String>>()?;
                        return Ok(Expr::Bool(nums[0] < nums[1]));
                    }

                    "<=" => {
                        if args.len() != 2 {
                            return Err("<= requires exactly 2 arguments".to_string());
                        }
                        let nums = args
                            .iter()
                            .map(|e| match e {
                                Expr::Number(n) => Ok(*n),
                                _ => Err("<= requires numbers".to_string()),
                            })
                            .collect::<Result<Vec<i32>, String>>()?;
                        return Ok(Expr::Bool(nums[0] <= nums[1]));
                    }

                    ">=" => {
                        if args.len() != 2 {
                            return Err(">= requires exactly 2 arguments".to_string());
                        }
                        let nums = args
                            .iter()
                            .map(|e| match e {
                                Expr::Number(n) => Ok(*n),
                                _ => Err(">= requires numbers".to_string()),
                            })
                            .collect::<Result<Vec<i32>, String>>()?;
                        return Ok(Expr::Bool(nums[0] >= nums[1]));
                    }

                    "=" => {
                        if args.len() != 2 {
                            return Err("= requires exactly 2 arguments".to_string());
                        }
                        let nums = args
                            .iter()
                            .map(|e| match e {
                                Expr::Number(n) => Ok(*n),
                                _ => Err("= requires numbers".to_string()),
                            })
                            .collect::<Result<Vec<i32>, String>>()?;
                        return Ok(Expr::Bool(nums[0] == nums[1]));
                    }
                    _ => Err(format!("unknown function: {}", func_name)),
                }
            } else {
                Err("first element must be a function name".to_string())
            }
        }
    }
}
