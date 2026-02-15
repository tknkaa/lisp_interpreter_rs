use lisp_interpreter_rs::*;

#[test]
fn test_full_pipeline_simple() {
    let input = "(+ 1 2)";
    let tokens = tokenize(input);
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(3));
}

#[test]
fn test_full_pipeline_complex() {
    let input = "(+ (* 2 3) (- 10 5))";
    let tokens = tokenize(input);
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(11));
}

#[test]
fn test_full_pipeline_deeply_nested() {
    let input = "(* (+ 1 2) (- 10 (* 2 3)))";
    let tokens = tokenize(input);
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(12));
}

#[test]
fn test_full_pipeline_with_define() {
    let input1 = "(define x 10)";
    let input2 = "(+ x 5)";

    let mut env = default_env();

    // First expression: define x
    let tokens1 = tokenize(input1);
    let token_refs1: Vec<&str> = tokens1.iter().map(|s| s.as_str()).collect();
    let expr1 = parse(&token_refs1).unwrap();
    eval(&expr1, &mut env).unwrap();

    // Second expression: use x
    let tokens2 = tokenize(input2);
    let token_refs2: Vec<&str> = tokens2.iter().map(|s| s.as_str()).collect();
    let expr2 = parse(&token_refs2).unwrap();
    let result = eval(&expr2, &mut env).unwrap();

    assert_eq!(result, Expr::Number(15));
}

#[test]
fn test_full_pipeline_multiple_defines() {
    let mut env = default_env();

    let inputs = vec![
        "(define a 5)",
        "(define b 10)",
        "(define c (+ a b))",
        "(* c 2)",
    ];

    let mut final_result = Expr::Number(0);

    for input in inputs {
        let tokens = tokenize(input);
        let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
        let expr = parse(&token_refs).unwrap();
        final_result = eval(&expr, &mut env).unwrap();
    }

    assert_eq!(final_result, Expr::Number(30)); // (5 + 10) * 2
}

#[test]
fn test_full_pipeline_division() {
    let input = "(/ (* 20 3) (+ 2 4))";
    let tokens = tokenize(input);
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(10)); // (20 * 3) / (2 + 4) = 60 / 6 = 10
}

#[test]
fn test_full_pipeline_all_operators() {
    let input = "(+ (- 100 10) (* 5 3) (/ 20 4))";
    let tokens = tokenize(input);
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(110)); // 90 + 15 + 5
}

#[test]
fn test_full_pipeline_negative_numbers() {
    let input = "(+ -5 10)";
    let tokens = tokenize(input);
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(5));
}

#[test]
fn test_full_pipeline_parse_error() {
    let input = "(+ 1 2"; // Missing closing paren
    let tokens = tokenize(input);
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let result = parse(&token_refs);
    assert!(result.is_err());
}

#[test]
fn test_full_pipeline_eval_error() {
    let input = "(unknown 1 2)";
    let tokens = tokenize(input);
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut default_env());
    assert!(result.is_err());
}

#[test]
fn test_full_pipeline_whitespace_handling() {
    let input = "  (  +   1    2   )  ";
    let tokens = tokenize(input);
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(3));
}

#[test]
fn test_full_pipeline_complex_with_variables() {
    let mut env = default_env();

    // Define variables
    let setup = vec!["(define x 5)", "(define y 3)"];

    for input in setup {
        let tokens = tokenize(input);
        let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
        let expr = parse(&token_refs).unwrap();
        eval(&expr, &mut env).unwrap();
    }

    // Complex expression using variables
    let input = "(* (+ x y) (- x y))";
    let tokens = tokenize(input);
    let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let expr = parse(&token_refs).unwrap();
    let result = eval(&expr, &mut env).unwrap();

    assert_eq!(result, Expr::Number(16)); // (5 + 3) * (5 - 3) = 8 * 2 = 16
}
