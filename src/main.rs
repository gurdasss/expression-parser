use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

mod lexer;
use lexer::Lexer;

fn main() -> Result<()> {
    // The DefaultEditor is a struct provided by the rustyline crate that implements a readline-like interface.
    // if this returns an Err, immediately return that error from the current function.
    // If it's Ok, unwrap it and give me the value.
    let mut rl = DefaultEditor::new()?;

    let mut lexer = Lexer::new("3 + 4 * 2");
    println!("{:?}", lexer.tokenize());

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
                        println!("{}", line.trim());
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
