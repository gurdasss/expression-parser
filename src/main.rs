use std::io::Write;

fn main() {
    // Rust's loop construct is an infinite loop that can be exited with a break statement.
    loop {
        print!(">> ");
        // Flush the output to ensure the prompt is displayed before waiting for input.
        std::io::stdout().flush().unwrap();
        let mut input = String::new();

        // Read a line of input from the user.
        // read_line returns Result<usize> where the usize is the number of bytes read.
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("");
                break;
            } // EOF reached, exit the loop
            Ok(_) => {
                // Process the input
                // Trim the input to remove any trailing newline characters.
                match input.trim() {
                    "exit" => break,
                    "help" => {}
                    _ => {
                        println!("{}", input.trim());
                    }
                }
            }
            Err(error) => {
                eprintln!("error: {}", error);
                break; // We got an error, let's end the loop
            }
        }
    }
}
