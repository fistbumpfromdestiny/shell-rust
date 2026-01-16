#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let builtins = vec!["exit", "echo", "type"];
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
                    Some(("type", args)) => find_type(args, &builtins),
                    Some((cmd, _)) => println!("{cmd}: command not found"),
                    None if trimmed == "exit" => return,
                    None => println!("{trimmed}: command not found"),
                }
            }
            Err(err) => println!("error: {err}"),
        };
    }
}

fn echo(args: &str) {
    println!("{args}");
}

fn find_type(args: &str, builtins: &Vec<&str>) {
    let args: Vec<&str> = args.trim().split_whitespace().collect();
    for arg in &args {
        if builtins.contains(arg) {
            println!("{arg} is a shell builtin");
        } else {
            println!("{arg}: not found")
        }
    }
    
}
