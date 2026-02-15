use lisp_interpreter_rs::*;

// Less than (<) tests
#[test]
fn test_less_than_true() {
    // (< 3 5) should return true
    let expr = Expr::List(vec![
        Expr::Symbol("<".to_string()),
        Expr::Number(3),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

#[test]
fn test_less_than_false() {
    // (< 5 3) should return false
    let expr = Expr::List(vec![
        Expr::Symbol("<".to_string()),
        Expr::Number(5),
        Expr::Number(3),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(false));
}

#[test]
fn test_less_than_equal_false() {
    // (< 5 5) should return false
    let expr = Expr::List(vec![
        Expr::Symbol("<".to_string()),
        Expr::Number(5),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(false));
}

// Greater than (>) tests
#[test]
fn test_greater_than_true() {
    // (> 5 3) should return true
    let expr = Expr::List(vec![
        Expr::Symbol(">".to_string()),
        Expr::Number(5),
        Expr::Number(3),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

#[test]
fn test_greater_than_false() {
    // (> 3 5) should return false
    let expr = Expr::List(vec![
        Expr::Symbol(">".to_string()),
        Expr::Number(3),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(false));
}

#[test]
fn test_greater_than_equal_false() {
    // (> 5 5) should return false
    let expr = Expr::List(vec![
        Expr::Symbol(">".to_string()),
        Expr::Number(5),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(false));
}

// Less than or equal (<=) tests
#[test]
fn test_less_equal_true_less() {
    // (<= 3 5) should return true
    let expr = Expr::List(vec![
        Expr::Symbol("<=".to_string()),
        Expr::Number(3),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

#[test]
fn test_less_equal_true_equal() {
    // (<= 5 5) should return true
    let expr = Expr::List(vec![
        Expr::Symbol("<=".to_string()),
        Expr::Number(5),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

#[test]
fn test_less_equal_false() {
    // (<= 5 3) should return false
    let expr = Expr::List(vec![
        Expr::Symbol("<=".to_string()),
        Expr::Number(5),
        Expr::Number(3),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(false));
}

// Greater than or equal (>=) tests
#[test]
fn test_greater_equal_true_greater() {
    // (>= 5 3) should return true
    let expr = Expr::List(vec![
        Expr::Symbol(">=".to_string()),
        Expr::Number(5),
        Expr::Number(3),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

#[test]
fn test_greater_equal_true_equal() {
    // (>= 5 5) should return true
    let expr = Expr::List(vec![
        Expr::Symbol(">=".to_string()),
        Expr::Number(5),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

#[test]
fn test_greater_equal_false() {
    // (>= 3 5) should return false
    let expr = Expr::List(vec![
        Expr::Symbol(">=".to_string()),
        Expr::Number(3),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(false));
}

// Equal (=) tests
#[test]
fn test_equal_true() {
    // (= 5 5) should return true
    let expr = Expr::List(vec![
        Expr::Symbol("=".to_string()),
        Expr::Number(5),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

#[test]
fn test_equal_false() {
    // (= 5 3) should return false
    let expr = Expr::List(vec![
        Expr::Symbol("=".to_string()),
        Expr::Number(5),
        Expr::Number(3),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(false));
}

#[test]
fn test_equal_negative_numbers() {
    // (= -5 -5) should return true
    let expr = Expr::List(vec![
        Expr::Symbol("=".to_string()),
        Expr::Number(-5),
        Expr::Number(-5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

// Comparison with expressions
#[test]
fn test_comparison_with_expression() {
    // (< (+ 1 2) 5) should return true (3 < 5)
    let expr = Expr::List(vec![
        Expr::Symbol("<".to_string()),
        Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1),
            Expr::Number(2),
        ]),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

#[test]
fn test_comparison_both_expressions() {
    // (= (+ 2 3) (* 1 5)) should return true (5 = 5)
    let expr = Expr::List(vec![
        Expr::Symbol("=".to_string()),
        Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(2),
            Expr::Number(3),
        ]),
        Expr::List(vec![
            Expr::Symbol("*".to_string()),
            Expr::Number(1),
            Expr::Number(5),
        ]),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

// Integration with if
#[test]
fn test_comparison_with_if() {
    // (if (< 3 5) 100 200) should return 100
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::List(vec![
            Expr::Symbol("<".to_string()),
            Expr::Number(3),
            Expr::Number(5),
        ]),
        Expr::Number(100),
        Expr::Number(200),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(100));
}

#[test]
fn test_comparison_with_if_false() {
    // (if (> 3 5) 100 200) should return 200
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::List(vec![
            Expr::Symbol(">".to_string()),
            Expr::Number(3),
            Expr::Number(5),
        ]),
        Expr::Number(100),
        Expr::Number(200),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(200));
}

// Comparison with variables
#[test]
fn test_comparison_with_variables() {
    let mut env = default_env();
    env.insert("x".to_string(), Expr::Number(10));
    env.insert("y".to_string(), Expr::Number(20));

    // (< x y) should return true (10 < 20)
    let expr = Expr::List(vec![
        Expr::Symbol("<".to_string()),
        Expr::Symbol("x".to_string()),
        Expr::Symbol("y".to_string()),
    ]);
    let result = eval(&expr, &mut env).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

// Comparison with zero
#[test]
fn test_comparison_with_zero() {
    // (> 5 0) should return true
    let expr = Expr::List(vec![
        Expr::Symbol(">".to_string()),
        Expr::Number(5),
        Expr::Number(0),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

#[test]
fn test_comparison_negative_numbers() {
    // (< -10 -5) should return true
    let expr = Expr::List(vec![
        Expr::Symbol("<".to_string()),
        Expr::Number(-10),
        Expr::Number(-5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Bool(true));
}

// Error cases
#[test]
fn test_comparison_insufficient_args() {
    // (< 5) should error
    let expr = Expr::List(vec![Expr::Symbol("<".to_string()), Expr::Number(5)]);
    let result = eval(&expr, &mut default_env());
    assert!(result.is_err());
}

#[test]
fn test_comparison_too_many_args() {
    // (< 1 2 3) should error
    let expr = Expr::List(vec![
        Expr::Symbol("<".to_string()),
        Expr::Number(1),
        Expr::Number(2),
        Expr::Number(3),
    ]);
    let result = eval(&expr, &mut default_env());
    assert!(result.is_err());
}

// Complex nested comparison
#[test]
fn test_nested_comparison_with_if() {
    // (if (= (+ 2 3) 5) (if (< 1 2) 42 0) 99) should return 42
    let expr = Expr::List(vec![
        Expr::Symbol("if".to_string()),
        Expr::List(vec![
            Expr::Symbol("=".to_string()),
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Number(2),
                Expr::Number(3),
            ]),
            Expr::Number(5),
        ]),
        Expr::List(vec![
            Expr::Symbol("if".to_string()),
            Expr::List(vec![
                Expr::Symbol("<".to_string()),
                Expr::Number(1),
                Expr::Number(2),
            ]),
            Expr::Number(42),
            Expr::Number(0),
        ]),
        Expr::Number(99),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(42));
}
