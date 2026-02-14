use lisp_interpreter_rs::*;

#[test]
fn test_define_and_lookup() {
    let mut env = default_env();

    // Define a variable
    let define_expr = Expr::List(vec![
        Expr::Symbol("define".to_string()),
        Expr::Symbol("x".to_string()),
        Expr::Number(42),
    ]);
    eval(&define_expr, &mut env).unwrap();

    // Lookup the variable
    let lookup_expr = Expr::Symbol("x".to_string());
    let result = eval(&lookup_expr, &mut env).unwrap();
    assert_eq!(result, Expr::Number(42));
}

#[test]
fn test_use_defined_variable() {
    let mut env = default_env();

    // Define a variable
    let define_expr = Expr::List(vec![
        Expr::Symbol("define".to_string()),
        Expr::Symbol("x".to_string()),
        Expr::Number(10),
    ]);
    eval(&define_expr, &mut env).unwrap();

    // Use it in an expression
    let expr = Expr::List(vec![
        Expr::Symbol("+".to_string()),
        Expr::Symbol("x".to_string()),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut env).unwrap();
    assert_eq!(result, Expr::Number(15));
}

#[test]
fn test_undefined_variable_error() {
    let expr = Expr::Symbol("undefined_var".to_string());
    let result = eval(&expr, &mut default_env());
    assert!(result.is_err());
}
