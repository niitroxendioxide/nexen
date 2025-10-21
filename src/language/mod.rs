use std::time::Instant;

use crate::language::errors::ProgramError;

pub mod tokens;
pub mod datatypes;
pub mod expressions;
pub mod errors;
pub mod scopes;
pub mod binder;
pub mod stdlib;
pub mod vm;
pub mod tokenizer;

pub fn interpret(source: String) -> Result<std::time::Duration, errors::ProgramError> {
    let mut program = tokens::Program::new();
    program.tokenize(&source);

    let program_time = Instant::now();
    match program.begin() {
        Err(err) => {
            let code_at_line = source.lines().nth(program.current_line - 1).unwrap_or("");
            
            Err(ProgramError::new(err.message, program.current_line, code_at_line.to_string()))
        }
        Ok(_) => return Ok(program_time.elapsed()),
    }
}

pub fn tokenize(source: String) -> Result<(), errors::LangError> {
    let mut program = tokens::Program::new();
    program.tokenize(&source);
    println!("{}", program);
    Ok(())
}

pub fn bytecode_compile(_source: String) -> Result<(), String> {
    let _m = vm::vm::NxVirtualMachine::new();
    //vm.run(source)
    // 
    
    Ok(())
}