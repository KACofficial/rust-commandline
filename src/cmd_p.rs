use std::process::Command;
use std::io;
use std::fs;
use colored::*;

pub fn process_command(command_f: &str) -> i32 { // i32 is the code
    let mut parts = command_f.split_whitespace();
    let command: &str = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();
    // println!("Running \'{}\' with args {:?}", command, args);
    match command {
        "clear" => {
            let _ = Command::new("clear").status();
            0
        }
        "ls" => {
            if let Err(err) = list_dir(if args.is_empty() { None } else { Some(&args) }) {
                eprintln!("Error listing directory: {}", err);
                1
            } else {
                0
            }
        }
        "cd" => {
            if args.is_empty() {
                eprintln!("No directory provided");
                1
            } else {
                if let Err(err) = std::env::set_current_dir(&args[0]) {
                    eprintln!("Error changing directory: {}", err);
                    1
                } else {
                    0
                }
            }
        }
        //TODO: add other commands
        "cat" => {
            if args.is_empty() {
                eprintln!("No file provided");
                1
            } else {
                if let Err(err) = cat_file(&args[0]) {
                    eprintln!("Error reading file: {}", err);
                    1
                } else {
                    0
                }
            }
        }
        _ => 1
    }
}



fn list_dir(args: Option<&[&str]>) -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let path: &str = match args {
        Some(a) if !a.is_empty() => a[0], // Use the first argument as directory path
        _ => current_dir.to_str().unwrap(), // Default to current directory
    };
    // println!("Directory to list: {}", path);

    let dir_contents = fs::read_dir(path)?;

    for content in dir_contents {
        let content = content?;
        let path = content.path();
        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                if path.is_dir(){
                    println!("{}/", file_name_str.blue());
                } else if path.is_symlink() {
                    match fs::read_link(&path) {
                        Ok(target_path) => {
                            if let Some(target_str) = target_path.to_str() {
                                println!("{} -> {}", file_name_str.yellow(), target_str.cyan());
                            } else {
                                println!("{} -> (invalid target)", file_name_str.yellow());
                            }
                        },
                        Err(e) => {
                            println!("{} -> (error reading target: {})", file_name_str.yellow(), e);
                        },
                    }
                } else {
                    println!("{}", file_name_str);
                }
            }
        }
    }

    Ok(())
}


fn cat_file(file_path: &str) -> io::Result<()> {
    let mut file = fs::File::open(file_path)?;
    io::copy(&mut file, &mut io::stdout())?;
    Ok(())
}