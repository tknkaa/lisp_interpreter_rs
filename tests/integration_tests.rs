use lisp_interpreter_rs::*;

#[test]
fn test_full_pipeline_simple() {
    let input = "(+ 1 2)";
    let tokens = tokenize(input);
    let expr = parse(&tokens).unwrap();
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(3));
}

#[test]
fn test_full_pipeline_complex() {
    let input = "(+ (* 2 3) (- 10 5))";
    let tokens = tokenize(input);
    let expr = parse(&tokens).unwrap();
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(11));
}

#[test]
fn test_full_pipeline_deeply_nested() {
    let input = "(* (+ 1 2) (- 10 (* 2 3)))";
    let tokens = tokenize(input);
    let expr = parse(&tokens).unwrap();
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(12));
}
