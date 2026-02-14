use lisp_interpreter_rs::*;

#[test]
fn test_tokenize_empty() {
    let tokens = tokenize("");
    assert_eq!(tokens, Vec::<String>::new());
}

#[test]
fn test_tokenize_single_atom() {
    let tokens = tokenize("42");
    assert_eq!(tokens, vec!["42"]);
}

#[test]
fn test_tokenize_symbol() {
    let tokens = tokenize("foo");
    assert_eq!(tokens, vec!["foo"]);
}

#[test]
fn test_tokenize_simple_list() {
    let tokens = tokenize("(+ 1 2)");
    assert_eq!(tokens, vec!["(", "+", "1", "2", ")"]);
}

#[test]
fn test_tokenize_nested_list() {
    let tokens = tokenize("(+ (* 2 3) 4)");
    assert_eq!(tokens, vec!["(", "+", "(", "*", "2", "3", ")", "4", ")"]);
}

#[test]
fn test_tokenize_with_whitespace() {
    let tokens = tokenize("  (  +   1   2  )  ");
    assert_eq!(tokens, vec!["(", "+", "1", "2", ")"]);
}

#[test]
fn test_tokenize_multiple_expressions() {
    let tokens = tokenize("(+ 1 2) (- 3 4)");
    assert_eq!(
        tokens,
        vec!["(", "+", "1", "2", ")", "(", "-", "3", "4", ")"]
    );
}

#[test]
fn test_tokenize_negative_number() {
    let tokens = tokenize("(+ -5 10)");
    assert_eq!(tokens, vec!["(", "+", "-5", "10", ")"]);
}

// ============= ADDITIONAL TESTS =============

#[test]
fn test_tokenize_only_spaces() {
    let tokens = tokenize("     ");
    assert_eq!(tokens, Vec::<String>::new());
}

#[test]
fn test_tokenize_tabs_and_newlines() {
    let tokens = tokenize("(\t+\n1\r\n2)");
    assert_eq!(tokens, vec!["(", "+", "1", "2", ")"]);
}

#[test]
fn test_tokenize_multiple_consecutive_parens() {
    let tokens = tokenize("(())");
    assert_eq!(tokens, vec!["(", "(", ")", ")"]);
}

#[test]
fn test_tokenize_no_spaces_between_parens() {
    let tokens = tokenize("((+ 1 2))");
    assert_eq!(tokens, vec!["(", "(", "+", "1", "2", ")", ")"]);
}

#[test]
fn test_tokenize_deeply_nested() {
    let tokens = tokenize("(((1)))");
    assert_eq!(tokens, vec!["(", "(", "(", "1", ")", ")", ")"]);
}

#[test]
fn test_tokenize_complex_expression() {
    let tokens = tokenize("(define square (lambda (x) (* x x)))");
    assert_eq!(
        tokens,
        vec![
            "(", "define", "square", "(", "lambda", "(", "x", ")", "(", "*", "x", "x", ")", ")",
            ")"
        ]
    );
}

#[test]
fn test_tokenize_mixed_whitespace() {
    let tokens = tokenize(" \t ( \n + \r 1   2 \t\n ) \r\n ");
    assert_eq!(tokens, vec!["(", "+", "1", "2", ")"]);
}

#[test]
fn test_tokenize_many_numbers() {
    let tokens = tokenize("(+ 1 2 3 4 5 6 7 8 9)");
    assert_eq!(
        tokens,
        vec!["(", "+", "1", "2", "3", "4", "5", "6", "7", "8", "9", ")"]
    );
}

#[test]
fn test_tokenize_long_symbol() {
    let tokens = tokenize("(very-long-symbol-name)");
    assert_eq!(tokens, vec!["(", "very-long-symbol-name", ")"]);
}

#[test]
fn test_tokenize_zero() {
    let tokens = tokenize("(+ 0 0)");
    assert_eq!(tokens, vec!["(", "+", "0", "0", ")"]);
}

#[test]
fn test_tokenize_large_numbers() {
    let tokens = tokenize("(+ 12345 67890)");
    assert_eq!(tokens, vec!["(", "+", "12345", "67890", ")"]);
}

#[test]
fn test_tokenize_symbols_with_special_chars() {
    let tokens = tokenize("(+ x-val y-val)");
    assert_eq!(tokens, vec!["(", "+", "x-val", "y-val", ")"]);
}

#[test]
fn test_tokenize_question_mark_in_symbol() {
    let tokens = tokenize("(even? 42)");
    assert_eq!(tokens, vec!["(", "even?", "42", ")"]);
}

#[test]
fn test_tokenize_empty_list() {
    let tokens = tokenize("()");
    assert_eq!(tokens, vec!["(", ")"]);
}

#[test]
fn test_tokenize_list_of_empty_lists() {
    let tokens = tokenize("(() () ())");
    assert_eq!(tokens, vec!["(", "(", ")", "(", ")", "(", ")", ")"]);
}

#[test]
fn test_tokenize_single_paren_open() {
    let tokens = tokenize("(");
    assert_eq!(tokens, vec!["("]);
}

#[test]
fn test_tokenize_single_paren_close() {
    let tokens = tokenize(")");
    assert_eq!(tokens, vec![")"]);
}

#[test]
fn test_tokenize_arithmetic_symbols() {
    let tokens = tokenize("+ - * /");
    assert_eq!(tokens, vec!["+", "-", "*", "/"]);
}

#[test]
fn test_tokenize_if_expression() {
    let tokens = tokenize("(if (> x 0) x (- x))");
    assert_eq!(
        tokens,
        vec![
            "(", "if", "(", ">", "x", "0", ")", "x", "(", "-", "x", ")", ")"
        ]
    );
}
