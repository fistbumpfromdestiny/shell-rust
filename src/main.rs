#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let mut command = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut command) {
            Ok(_) => {
                let trimmed = command.trim();
                if trimmed.is_empty() {
                    continue;
                }

                match trimmed.split_once(' ') {
                    Some(("exit", _)) => return,
                    Some(("echo", args)) => echo(args),
                    Some((cmd, _)) => println!("{cmd}: command not found"),
                    None if trimmed == "exit" => return,
                    None => println!("{trimmed}: command not found"),
                }
            }
            Err(err) => println!("error: {err}"),
        };
    }
}

fn echo(command: 
    &str) {
    println!("{command}");
}
