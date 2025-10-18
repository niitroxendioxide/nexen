use lang::language;
use lang::files;
use std::env;

struct ProgramParams {
    tokenize: bool,
    debug: bool,
}

fn parse_params(args: &Vec<String>, params: &mut ProgramParams) {
    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "-t" | "--tokenize" => params.tokenize = true,
            "-d" | "--debug" => params.debug = true,
            _ => println!("Unknown argument: {}", arg),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut params = ProgramParams {
        tokenize: false,
        debug: false,
    };
    
    if args.len() > 1 {
        parse_params(&args, &mut params);
        let file_path = &args[1];

        match files::file::validate_and_read_file(file_path) {
            Ok(source) => {
                if params.tokenize {
                    if let Err(err) = language::tokenize(source) {
                        println!("[Interpreter] when tokenizing {}: \n\n> {}", file_path, err)
                    }
                } else {
                    match language::interpret(source) {
                        Ok(program_exec_time) => {
                            if params.debug {
                                println!("\r\x1b[1;32m[Nexen]\x1b[0m Program finished\n-> Execution time: \x1b[1;31m[{:?}]\x1b[0m", program_exec_time);
                            }
                        }
                        Err(err) =>println!("[Interpreter] when executing {}: \n\n> {}", file_path, err)
                    }
                }

            },
            Err(err) => println!("{}", err),
        }
    } else {
        println!("No argument was provided.");
    }
}
