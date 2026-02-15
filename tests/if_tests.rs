use lisp_interpreter_rs::*;

#[test]
fn test_if_true_condition() {
    // (if true 10 20) should return 10
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::Bool(true),
        Expr::Number(10),
        Expr::Number(20),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(10));
}

#[test]
fn test_if_false_condition() {
    // (if false 10 20) should return 20
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::Bool(false),
        Expr::Number(10),
        Expr::Number(20),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(20));
}

#[test]
fn test_if_with_expression_in_then() {
    // (if true (+ 5 5) 0) should return 10
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::Bool(true),
        Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(5),
            Expr::Number(5),
        ]),
        Expr::Number(0),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(10));
}

#[test]
fn test_if_with_expression_in_else() {
    // (if false 0 (* 3 4)) should return 12
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::Bool(false),
        Expr::Number(0),
        Expr::List(vec![
            Expr::Symbol("*".to_string()),
            Expr::Number(3),
            Expr::Number(4),
        ]),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(12));
}

#[test]
fn test_if_nested() {
    // (if true (if false 1 2) 3) should return 2
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::Bool(true),
        Expr::List(vec![
            Expr::Symbol("if".to_string()),
            Expr::Bool(false),
            Expr::Number(1),
            Expr::Number(2),
        ]),
        Expr::Number(3),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(2));
}

#[test]
fn test_if_with_variable() {
    // (define x 10) (if true x 20) should return 10
    let mut env = default_env();
    env.insert("x".to_string(), Expr::Number(10));

    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::Bool(true),
        Expr::Symbol("x".to_string()),
        Expr::Number(20),
    ]);
    let result = eval(&expr, &mut env).unwrap();
    assert_eq!(result, Expr::Number(10));
}

#[test]
fn test_if_returns_bool() {
    // (if true true false) should return true
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::Bool(true),
        Expr::Bool(true),
        Expr::Bool(false),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

#[test]
fn test_if_complex_nested() {
    // (if false (+ 1 2) (if true (* 2 3) (/ 10 2))) should return 6
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::Bool(false),
        Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1),
            Expr::Number(2),
        ]),
        Expr::List(vec![
            Expr::Symbol("if".to_string()),
            Expr::Bool(true),
            Expr::List(vec![
                Expr::Symbol("*".to_string()),
                Expr::Number(2),
                Expr::Number(3),
            ]),
            Expr::List(vec![
                Expr::Symbol("/".to_string()),
                Expr::Number(10),
                Expr::Number(2),
            ]),
        ]),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(6));
}

#[test]
fn test_if_insufficient_args_error() {
    // (if true 10) should error - missing else branch
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::Bool(true),
        Expr::Number(10),
    ]);
    let result = eval(&expr, &mut default_env());
    assert!(result.is_err());
}

#[test]
fn test_if_too_many_args_error() {
    // (if true 10 20 30) should error - too many arguments
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::Bool(true),
        Expr::Number(10),
        Expr::Number(20),
        Expr::Number(30),
    ]);
    let result = eval(&expr, &mut default_env());
    assert!(result.is_err());
}

#[test]
fn test_if_with_define_in_branches() {
    // Test that if evaluates only the taken branch
    // This ensures lazy evaluation
    let mut env = default_env();

    // (if true 42 (define should_not_run 999))
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::Bool(true),
        Expr::Number(42),
        Expr::List(vec![
            Expr::Symbol("define".to_string()),
            Expr::Symbol("should_not_run".to_string()),
            Expr::Number(999),
        ]),
    ]);
    let result = eval(&expr, &mut env).unwrap();
    assert_eq!(result, Expr::Number(42));
    // should_not_run should not be defined
    assert!(!env.contains_key("should_not_run"));
}

#[test]
fn test_if_deeply_nested_multiple_levels() {
    // (if true (if true (if false 1 2) 3) 4) should return 2
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::Bool(true),
        Expr::List(vec![
            Expr::Symbol("if".to_string()),
            Expr::Bool(true),
            Expr::List(vec![
                Expr::Symbol("if".to_string()),
                Expr::Bool(false),
                Expr::Number(1),
                Expr::Number(2),
            ]),
            Expr::Number(3),
        ]),
        Expr::Number(4),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(2));
}
