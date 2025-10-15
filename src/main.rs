use lang::language;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the argument exists and access it
    if args.len() > 1 {
        let first_arg = &args[1];
        language::tokens::interpret(first_arg);
    } else {
        println!("No argument was provided.");
    }

}