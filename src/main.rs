use lang::language;
use lang::files;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        let tokenize = if args.len() > 2 && args[2] == "-t" {
            true
        } else { false };
        
        match files::file::validate_and_read_file(file_path) {
            Ok(source) => {
                if tokenize {
                    if let Err(err) = language::tokenize(source) {
                        println!("[NX-Interpreter] when tokenizing {}: \n\n> {}", file_path, err)
                    }
                } else {
                    if let Err(err) = language::interpret(source) {
                        println!("[NX-Interpreter] when executing {}: \n\n> {}", file_path, err)
                    }
                }
            },
            Err(err) => println!("{}", err),
        }
    } else {
        println!("No argument was provided.");
    }

}