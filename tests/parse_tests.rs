use lisp_interpreter_rs::*;

#[test]
fn test_parse_number() {
    let tokens = vec!["42"];
    let expr = parse(&tokens).unwrap();
    assert_eq!(expr, Expr::Number(42));
}

#[test]
fn test_parse_symbol() {
    let tokens = vec!["foo"];
    let expr = parse(&tokens).unwrap();
    assert_eq!(expr, Expr::Symbol("foo".to_string()));
}

#[test]
fn test_parse_empty_list() {
    let tokens = vec!["(", ")"];
    let expr = parse(&tokens).unwrap();
    assert_eq!(expr, Expr::List(vec![]));
}

#[test]
fn test_parse_simple_list() {
    let tokens = vec!["(", "+", "1", "2", ")"];
    let expr = parse(&tokens).unwrap();
    assert_eq!(
        expr,
        Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1),
            Expr::Number(2)
        ])
    );
}

#[test]
fn test_parse_nested_list() {
    let tokens = vec!["(", "+", "(", "*", "2", "3", ")", "4", ")"];
    let expr = parse(&tokens).unwrap();
    assert_eq!(
        expr,
        Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::List(vec![
                Expr::Symbol("*".to_string()),
                Expr::Number(2),
                Expr::Number(3)
            ]),
            Expr::Number(4)
        ])
    );
}

#[test]
fn test_parse_error_unmatched_paren() {
    let tokens = vec!["(", "+", "1", "2"];
    let result = parse(&tokens);
    assert!(result.is_err());
}

#[test]
fn test_parse_error_extra_closing_paren() {
    let tokens = vec![")", "+", "1", "2"];
    let result = parse(&tokens);
    assert!(result.is_err());
}
