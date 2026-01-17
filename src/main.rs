#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

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
                    Some((cmd, args)) => locate_exec(cmd, args, false),
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
            let args: &str = "";
            locate_exec(arg, args, true)
        }
    }
}

fn locate_exec(arg: &str, args: &str, find_type: bool) {
    if let Some(paths) = env::var_os("PATH") {
        let mut found = false;

        for path in env::split_paths(&paths) {
            let full_path = path.join(arg);
            if full_path.exists() {
                if let Ok(metadata) = full_path.metadata() {
                    let permissions = metadata.permissions();

                    if permissions.mode() & 0o111 != 0 {
                        if find_type {
                            println!("{arg} is {}", full_path.display());
                        } else {
                            let cmd_args: Vec<&str> = args.split_whitespace().collect();

                            match Command::new(arg).args(&cmd_args).output() {
                                Ok(output) => {
                                    print!("{}", String::from_utf8_lossy(&output.stdout));
                                    print!("{}", String::from_utf8_lossy(&output.stderr));
                                }
                                Err(error) => {
                                    println!("{}", error);
                                }
                            }
                        }

                        found = true;
                        break;
                    }
                }
            }
        }

        if !found {
            println!("{arg}: not found");
        }
    } else {
        println!("{arg}: not found");
    }
}
