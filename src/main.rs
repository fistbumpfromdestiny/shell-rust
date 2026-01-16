#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let mut command = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut command) {
            Ok(_) if command.is_empty() => continue,
            Ok(_) if command.trim() == "exit" => return,
            Ok(_) => println!("{}: command not found", command.trim()),
            Err(err) => println!("error: {err}"),
        };
    }
}
