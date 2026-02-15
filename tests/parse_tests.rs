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
    println!("result: {:?}", result);
    assert!(result.is_err());
}

#[test]
fn test_parse_error_extra_closing_paren() {
    let tokens = vec![")", "+", "1", "2"];
    let result = parse(&tokens);
    assert!(result.is_err());
}

#[test]
fn test_parse_error_empty_tokens() {
    let tokens: Vec<&str> = vec![];
    let result = parse(&tokens);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "no tokens to parse");
}

#[test]
fn test_parse_error_unexpected_tokens_after() {
    let tokens = vec!["42", "extra"];
    let result = parse(&tokens);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("unexpected token"));
}

#[test]
fn test_parse_deeply_nested_list() {
    let tokens = vec!["(", "(", "(", "a", ")", ")", ")"];
    let expr = parse(&tokens).unwrap();
    assert_eq!(
        expr,
        Expr::List(vec![Expr::List(vec![Expr::List(vec![Expr::Symbol(
            "a".to_string()
        )])])])
    );
}

#[test]
fn test_parse_multiple_numbers() {
    let tokens = vec!["(", "1", "2", "3", "4", "5", ")"];
    let expr = parse(&tokens).unwrap();
    assert_eq!(
        expr,
        Expr::List(vec![
            Expr::Number(1),
            Expr::Number(2),
            Expr::Number(3),
            Expr::Number(4),
            Expr::Number(5)
        ])
    );
}

#[test]
fn test_parse_mixed_symbols_and_numbers() {
    let tokens = vec!["(", "define", "x", "100", ")"];
    let expr = parse(&tokens).unwrap();
    assert_eq!(
        expr,
        Expr::List(vec![
            Expr::Symbol("define".to_string()),
            Expr::Symbol("x".to_string()),
            Expr::Number(100)
        ])
    );
}

#[test]
fn test_parse_list_with_symbols_only() {
    let tokens = vec!["(", "foo", "bar", "baz", ")"];
    let expr = parse(&tokens).unwrap();
    assert_eq!(
        expr,
        Expr::List(vec![
            Expr::Symbol("foo".to_string()),
            Expr::Symbol("bar".to_string()),
            Expr::Symbol("baz".to_string())
        ])
    );
}

#[test]
fn test_parse_error_unclosed_nested_list() {
    let tokens = vec!["(", "+", "(", "*", "2", "3", ")"];
    let result = parse(&tokens);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("unclosed list"));
}

#[test]
fn test_parse_complex_nested_expression() {
    let tokens = vec![
        "(", "if", "(", ">", "x", "0", ")", "(", "+", "x", "1", ")", "(", "-", "x", "1", ")", ")",
    ];
    let expr = parse(&tokens).unwrap();
    assert_eq!(
        expr,
        Expr::List(vec![
            Expr::Symbol("if".to_string()),
            Expr::List(vec![
                Expr::Symbol(">".to_string()),
                Expr::Symbol("x".to_string()),
                Expr::Number(0)
            ]),
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Symbol("x".to_string()),
                Expr::Number(1)
            ]),
            Expr::List(vec![
                Expr::Symbol("-".to_string()),
                Expr::Symbol("x".to_string()),
                Expr::Number(1)
            ])
        ])
    );
}
