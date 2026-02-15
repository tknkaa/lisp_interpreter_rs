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

#[test]
fn test_redefine_variable() {
    let mut env = default_env();

    // Define x = 10
    let define_expr = Expr::List(vec![
        Expr::Symbol("define".to_string()),
        Expr::Symbol("x".to_string()),
        Expr::Number(10),
    ]);
    eval(&define_expr, &mut env).unwrap();

    // Redefine x = 20
    let redefine_expr = Expr::List(vec![
        Expr::Symbol("define".to_string()),
        Expr::Symbol("x".to_string()),
        Expr::Number(20),
    ]);
    eval(&redefine_expr, &mut env).unwrap();

    // Lookup should return new value
    let lookup_expr = Expr::Symbol("x".to_string());
    let result = eval(&lookup_expr, &mut env).unwrap();
    assert_eq!(result, Expr::Number(20));
}

#[test]
fn test_multiple_variables() {
    let mut env = default_env();

    // Define multiple variables
    eval(
        &Expr::List(vec![
            Expr::Symbol("define".to_string()),
            Expr::Symbol("a".to_string()),
            Expr::Number(5),
        ]),
        &mut env,
    )
    .unwrap();

    eval(
        &Expr::List(vec![
            Expr::Symbol("define".to_string()),
            Expr::Symbol("b".to_string()),
            Expr::Number(10),
        ]),
        &mut env,
    )
    .unwrap();

    eval(
        &Expr::List(vec![
            Expr::Symbol("define".to_string()),
            Expr::Symbol("c".to_string()),
            Expr::Number(15),
        ]),
        &mut env,
    )
    .unwrap();

    // Use all variables
    let expr = Expr::List(vec![
        Expr::Symbol("+".to_string()),
        Expr::Symbol("a".to_string()),
        Expr::Symbol("b".to_string()),
        Expr::Symbol("c".to_string()),
    ]);
    let result = eval(&expr, &mut env).unwrap();
    assert_eq!(result, Expr::Number(30));
}

#[test]
fn test_define_with_expression() {
    let mut env = default_env();

    // Define x = (+ 2 3)
    let define_expr = Expr::List(vec![
        Expr::Symbol("define".to_string()),
        Expr::Symbol("x".to_string()),
        Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(2),
            Expr::Number(3),
        ]),
    ]);
    eval(&define_expr, &mut env).unwrap();

    // Lookup x
    let lookup_expr = Expr::Symbol("x".to_string());
    let result = eval(&lookup_expr, &mut env).unwrap();
    assert_eq!(result, Expr::Number(5));
}

#[test]
fn test_variable_in_nested_expression() {
    let mut env = default_env();

    // Define x = 3
    eval(
        &Expr::List(vec![
            Expr::Symbol("define".to_string()),
            Expr::Symbol("x".to_string()),
            Expr::Number(3),
        ]),
        &mut env,
    )
    .unwrap();

    // (+ (* x 4) 5) = (+ (* 3 4) 5) = (+ 12 5) = 17
    let expr = Expr::List(vec![
        Expr::Symbol("+".to_string()),
        Expr::List(vec![
            Expr::Symbol("*".to_string()),
            Expr::Symbol("x".to_string()),
            Expr::Number(4),
        ]),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut env).unwrap();
    assert_eq!(result, Expr::Number(17));
}

#[test]
fn test_define_using_previous_variable() {
    let mut env = default_env();

    // Define x = 10
    eval(
        &Expr::List(vec![
            Expr::Symbol("define".to_string()),
            Expr::Symbol("x".to_string()),
            Expr::Number(10),
        ]),
        &mut env,
    )
    .unwrap();

    // Define y = (+ x 5) = 15
    eval(
        &Expr::List(vec![
            Expr::Symbol("define".to_string()),
            Expr::Symbol("y".to_string()),
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Symbol("x".to_string()),
                Expr::Number(5),
            ]),
        ]),
        &mut env,
    )
    .unwrap();

    // Use both
    let expr = Expr::List(vec![
        Expr::Symbol("*".to_string()),
        Expr::Symbol("x".to_string()),
        Expr::Symbol("y".to_string()),
    ]);
    let result = eval(&expr, &mut env).unwrap();
    assert_eq!(result, Expr::Number(150)); // 10 * 15
}

#[test]
fn test_define_negative_number() {
    let mut env = default_env();

    // Define x = -42
    eval(
        &Expr::List(vec![
            Expr::Symbol("define".to_string()),
            Expr::Symbol("x".to_string()),
            Expr::Number(-42),
        ]),
        &mut env,
    )
    .unwrap();

    let lookup_expr = Expr::Symbol("x".to_string());
    let result = eval(&lookup_expr, &mut env).unwrap();
    assert_eq!(result, Expr::Number(-42));
}
