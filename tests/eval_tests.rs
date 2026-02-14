use lisp_interpreter_rs::*;

#[test]
fn test_eval_number() {
    let expr = Expr::Number(42);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(42));
}

#[test]
fn test_eval_addition() {
    let expr = Expr::List(vec![
        Expr::Symbol("+".to_string()),
        Expr::Number(1),
        Expr::Number(2),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(3));
}

#[test]
fn test_eval_subtraction() {
    let expr = Expr::List(vec![
        Expr::Symbol("-".to_string()),
        Expr::Number(10),
        Expr::Number(3),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(7));
}

#[test]
fn test_eval_multiplication() {
    let expr = Expr::List(vec![
        Expr::Symbol("*".to_string()),
        Expr::Number(3),
        Expr::Number(4),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(12));
}

#[test]
fn test_eval_division() {
    let expr = Expr::List(vec![
        Expr::Symbol("/".to_string()),
        Expr::Number(20),
        Expr::Number(4),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(5));
}

#[test]
fn test_eval_nested_expression() {
    let expr = Expr::List(vec![
        Expr::Symbol("+".to_string()),
        Expr::List(vec![
            Expr::Symbol("*".to_string()),
            Expr::Number(2),
            Expr::Number(3),
        ]),
        Expr::Number(4),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(10));
}

#[test]
fn test_eval_multiple_args() {
    let expr = Expr::List(vec![
        Expr::Symbol("+".to_string()),
        Expr::Number(1),
        Expr::Number(2),
        Expr::Number(3),
        Expr::Number(4),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(10));
}

#[test]
fn test_eval_empty_list_error() {
    let expr = Expr::List(vec![]);
    let result = eval(&expr, &mut default_env());
    assert!(result.is_err());
}

#[test]
fn test_eval_unknown_function() {
    let expr = Expr::List(vec![Expr::Symbol("unknown".to_string()), Expr::Number(1)]);
    let result = eval(&expr, &mut default_env());
    assert!(result.is_err());
}
