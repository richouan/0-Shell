use std::io::{self, Write};
mod commands;
use commands::echo;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();

        if bytes_read == 0 {
            println!();
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match cmd {
            "exit" => break,
            "echo" => echo::run(&args),
            _ => eprintln!("Command '{}' not found", cmd),
        }
    }
}
