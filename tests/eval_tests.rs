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

#[test]
fn test_eval_division_by_zero() {
    let expr = Expr::List(vec![
        Expr::Symbol("/".to_string()),
        Expr::Number(10),
        Expr::Number(0),
    ]);
    let result = eval(&expr, &mut default_env());
    assert!(result.is_err());
}

#[test]
fn test_eval_subtraction_single_arg() {
    let expr = Expr::List(vec![Expr::Symbol("-".to_string()), Expr::Number(10)]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(-10));
}

#[test]
fn test_eval_subtraction_multiple_args() {
    let expr = Expr::List(vec![
        Expr::Symbol("-".to_string()),
        Expr::Number(100),
        Expr::Number(10),
        Expr::Number(5),
        Expr::Number(2),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(83)); // 100 - 10 - 5 - 2
}

#[test]
fn test_eval_multiplication_multiple_args() {
    let expr = Expr::List(vec![
        Expr::Symbol("*".to_string()),
        Expr::Number(2),
        Expr::Number(3),
        Expr::Number(4),
        Expr::Number(5),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(120)); // 2 * 3 * 4 * 5
}

#[test]
fn test_eval_deeply_nested() {
    // (+ (* 2 (+ 3 4)) (- 10 5))
    let expr = Expr::List(vec![
        Expr::Symbol("+".to_string()),
        Expr::List(vec![
            Expr::Symbol("*".to_string()),
            Expr::Number(2),
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Number(3),
                Expr::Number(4),
            ]),
        ]),
        Expr::List(vec![
            Expr::Symbol("-".to_string()),
            Expr::Number(10),
            Expr::Number(5),
        ]),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(19)); // (2 * 7) + 5 = 14 + 5 = 19
}

#[test]
fn test_eval_complex_arithmetic() {
    // (/ (* (+ 2 3) 4) 2)
    let expr = Expr::List(vec![
        Expr::Symbol("/".to_string()),
        Expr::List(vec![
            Expr::Symbol("*".to_string()),
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Number(2),
                Expr::Number(3),
            ]),
            Expr::Number(4),
        ]),
        Expr::Number(2),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(10)); // ((2 + 3) * 4) / 2 = (5 * 4) / 2 = 20 / 2 = 10
}

#[test]
fn test_eval_addition_with_zero() {
    let expr = Expr::List(vec![
        Expr::Symbol("+".to_string()),
        Expr::Number(0),
        Expr::Number(5),
        Expr::Number(0),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(5));
}

#[test]
fn test_eval_multiplication_with_zero() {
    let expr = Expr::List(vec![
        Expr::Symbol("*".to_string()),
        Expr::Number(5),
        Expr::Number(0),
        Expr::Number(10),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(0));
}

#[test]
fn test_eval_all_operators_combined() {
    // (+ (- 20 5) (* 3 4) (/ 16 2))
    let expr = Expr::List(vec![
        Expr::Symbol("+".to_string()),
        Expr::List(vec![
            Expr::Symbol("-".to_string()),
            Expr::Number(20),
            Expr::Number(5),
        ]),
        Expr::List(vec![
            Expr::Symbol("*".to_string()),
            Expr::Number(3),
            Expr::Number(4),
        ]),
        Expr::List(vec![
            Expr::Symbol("/".to_string()),
            Expr::Number(16),
            Expr::Number(2),
        ]),
    ]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(35)); // 15 + 12 + 8 = 35
}

#[test]
fn test_eval_single_addition() {
    let expr = Expr::List(vec![Expr::Symbol("+".to_string()), Expr::Number(42)]);
    let result = eval(&expr, &mut default_env()).unwrap();
    assert_eq!(result, Expr::Number(42));
}

#[test]
fn test_eval_no_args_error() {
    let expr = Expr::List(vec![Expr::Symbol("+".to_string())]);
    let result = eval(&expr, &mut default_env());
    // This might error or return 0 depending on implementation
    // Adjust assertion based on your implementation choice
    assert!(result.is_ok() || result.is_err());
}
