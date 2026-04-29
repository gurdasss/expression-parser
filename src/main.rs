//! # Expression Parser
//!
//! A command-line mathematical expression parser and evaluator.
//!
//! This program provides a REPL (Read-Eval-Print Loop) interface for evaluating
//! mathematical expressions. It uses a modular architecture with separate
//! lexer, parser, and evaluator components.
//!
//! ## Features
//!
//! - Lexical analysis of arithmetic expressions
//! - Pratt parser for operator precedence
//! - Interactive REPL with command history
//! - Error handling and reporting
//!
//! ## Usage
//!
//! Run the program and enter expressions at the `>> ` prompt.
//! Type `exit` to quit, `help` for assistance.

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

mod error;
mod evaluator;
mod expr;
mod lexer;
mod parser;
mod token;

use evaluator::eval;
use lexer::Lexer;
use parser::Parser;

/// The main entry point of the expression parser.
///
/// Initializes the REPL interface using rustyline for enhanced line editing
/// and history. Currently demonstrates basic tokenization; full expression
/// evaluation is under development.
fn main() -> Result<()> {
    // The DefaultEditor is a struct provided by the rustyline crate that implements a readline-like interface.
    // if this returns an Err, immediately return that error from the current function.
    // If it's Ok, unwrap it and give me the value.
    let mut rl = DefaultEditor::new()?;

    // Try to load the history from a file, if it fails, print a message but don't panic.
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    // Rust's loop construct is an infinite loop that can be exited with a break statement.
    loop {
        let input = rl.readline(">> ");
        match input {
            Ok(line) => {
                // Process the input
                // Trim the input to remove any trailing newline characters.
                match line.trim() {
                    "exit" => break,
                    "help" => {}
                    _ => {
                        let tokens = Lexer::new(&line).tokenize();
                        match Parser::new(tokens).parse() {
                            Ok(expr) => match eval(&expr) {
                                Ok(result) => println!("{}", result),
                                Err(e) => eprintln!("error: {}", e),
                            },
                            Err(e) => eprintln!("error: {}", e),
                        }
                    }
                }

                // try to add history entry, if it fails, print an error message but don't panic.
                if let Err(err) = rl.add_history_entry(line.as_str()) {
                    eprintln!("failed to add history entry: {}", err);
                }
            }
            Err(ReadlineError::Eof) => {
                println!("EOF");
                break;
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
                break;
            }
            Err(error) => {
                eprintln!("error: {}", error);
                break; // We got an error, let's end the loop
            }
        }
    }

    // Try to save the history, if it fails, print an error message but don't panic.
    #[cfg(feature = "with-file-history")]
    if let Err(err) = rl.save_history("history.txt") {
        eprintln!("failed to save history: {}", err);
    }

    // The main function returns a Result type, which is a common way to handle errors in Rust.
    Ok(())
}
