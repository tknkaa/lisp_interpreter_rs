use lisp_interpreter_rs::{Expr, default_env, eval, parse, tokenize};

#[test]
fn test_define_simple_function() {
    // (lambda (x) (* x 2))
    let mut env = default_env();
    let tokens = tokenize("(lambda (x) (* x 2))");
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();

    // Evaluate to get Lambda
    let result = eval(&expr, &mut env).unwrap();

    match result {
        Expr::Lambda { params, .. } => {
            assert_eq!(params, vec!["x"]);
        }
        _ => panic!("Expected Lambda, got {:?}", result),
    }
}

#[test]
fn test_call_user_function() {
    // (define double (lambda (x) (* x 2)))
    // (double 5) => 10
    let mut env = default_env();

    let tokens = tokenize("(define double (lambda (x) (* x 2)))");
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    eval(&expr, &mut env).unwrap();

    let tokens = tokenize("(double 5)");
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut env).unwrap();

    assert_eq!(result, Expr::Number(10));
}

#[test]
fn test_multi_param_function() {
    // (define add (lambda (x y) (+ x y)))
    // (add 3 7) => 10
    let mut env = default_env();

    let tokens = tokenize("(define add (lambda (x y) (+ x y)))");
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    eval(&expr, &mut env).unwrap();

    let tokens = tokenize("(add 3 7)");
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut env).unwrap();

    assert_eq!(result, Expr::Number(10));
}

#[test]
fn test_closure() {
    // (define make-adder (lambda (x) (lambda (y) (+ x y))))
    // (define add5 (make-adder 5))
    // (add5 10) => 15
    let mut env = default_env();

    let tokens = tokenize("(define make-adder (lambda (x) (lambda (y) (+ x y))))");
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    eval(&expr, &mut env).unwrap();

    let tokens = tokenize("(define add5 (make-adder 5))");
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    eval(&expr, &mut env).unwrap();

    let tokens = tokenize("(add5 10)");
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut env).unwrap();

    assert_eq!(result, Expr::Number(15));
}

#[test]
fn test_recursive_function() {
    // (define factorial (lambda (n) (if (<= n 1) 1 (* n (factorial (- n 1))))))
    // (factorial 5) => 120
    let mut env = default_env();

    let tokens =
        tokenize("(define factorial (lambda (n) (if (<= n 1) 1 (* n (factorial (- n 1))))))");
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    eval(&expr, &mut env).unwrap();

    let tokens = tokenize("(factorial 5)");
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut env).unwrap();

    assert_eq!(result, Expr::Number(120));
}
