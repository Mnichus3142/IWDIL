use std::env;
use git2::Repository;

mod help;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    println!("Args: {}", args_len);

    if args_len >= 2 {
        println!("Too many arguments provided.");
        println!("---------------------------------");
        help::create_help();
        return;
    }

    else if args_len > 1 {
        println!("Argument: {}", args[1]);
        return;        
    }

    else {
        help::create_help();
        return;
    }
}
