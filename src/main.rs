use lisp_interpreter_rs::*;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // Read the file
    let contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", file_path, err);
        process::exit(1);
    });

    // Create environment
    let mut env = default_env();

    // Process each line/expression
    for (line_num, line) in contents.lines().enumerate() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with(';') {
            continue;
        }

        // Tokenize
        let tokens = tokenize(line);
        if tokens.is_empty() {
            continue;
        }

        // Parse
        let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
        let expr = match parse(&token_refs) {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Parse error at line {}: {}", line_num + 1, err);
                process::exit(1);
            }
        };

        // Evaluate
        let result = match eval(&expr, &mut env) {
            Ok(r) => r,
            Err(err) => {
                eprintln!("Eval error at line {}: {}", line_num + 1, err);
                process::exit(1);
            }
        };

        // Print result
        println!("{}", result);
    }
}
