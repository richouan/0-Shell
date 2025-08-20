use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();

        // Handle EOF: Ctrl+D on Linux, Ctrl+Z on Windows
        if bytes_read == 0 || input.trim().is_empty() {
            println!();
            println!("exit");
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        println!("You typed: {}", input);
    }
}
