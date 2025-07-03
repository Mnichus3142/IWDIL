use std::env;
// use git2::Repository;
// use rusqlite::{params, Connection, Result};

mod help;
mod init;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len > 2 {
        println!("Too many arguments provided.");
        println!("---------------------------------");
        help::create_help();
        return;
    }

    else if args_len > 1 {
        match args[1].as_str() {
            "help" => {
                help::create_help();
                return;
            }
            "init" => {
                match init::init() {
                    Ok(_) => {
                        println!("Database initialized successfully.");
                    }
                    Err(e) => {
                        println!("Error initializing database: {}", e);
                    }
                }
                return;
            }
            _ => {
                println!("Unknown command: {}", args[1]);
                println!("---------------------------------");
                help::create_help();
                return;
            }
        }
    }

    else {
        help::create_help();
        return;
    }
}
