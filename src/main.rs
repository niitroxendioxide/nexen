use lang::language;
use lang::files;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];

        match files::file::validate_and_read_file(file_path) {
            Ok(source) => {
                if let Err(err) = language::interpret(source) {
                    println!("[NX-Interpreter] when executing .nx file: \n\n> {}", err)
                }
            },
            Err(err) => println!("{}", err),
        }
    } else {
        println!("No argument was provided.");
    }

}