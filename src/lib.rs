// Lisp Interpreter
// This module provides tokenization, parsing, and evaluation for a Lisp-like language

#[cfg(test)]
mod tests {
    use super::*;

    // ============= TOKENIZATION TESTS =============
    #[test]
    fn test_tokenize_empty() {
        let tokens = tokenize("");
        assert_eq!(tokens, vec![]);
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

    // ============= PARSING TESTS =============
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

    // ============= EVALUATION TESTS =============
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

    // ============= INTEGRATION TESTS =============
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

    // ============= ENVIRONMENT TESTS =============
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
}
