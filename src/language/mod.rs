use std::time::Instant;

pub mod tokens;
pub mod datatypes;
pub mod expressions;
pub mod errors;
pub mod scopes;
pub mod binder;
pub mod stdlib;
pub mod vm;
pub mod tokenizer;

pub fn interpret(source: String) -> Result<std::time::Duration, errors::LangError> {
    let mut program = tokens::Program::new();
    program.tokenize(source);

    let program_time = Instant::now();
    match program.begin() {
        Err(err) => return Err(err),
        Ok(_) => return Ok(program_time.elapsed()),
    }
}

pub fn tokenize(source: String) -> Result<(), errors::LangError> {
    let mut program = tokens::Program::new();
    program.tokenize(source);
    println!("{}", program);
    Ok(())
}

pub fn bytecode_compile(source: String) -> Result<(), String> {
    let vm = vm::vm::NxVirtualMachine::new();
    //vm.run(source)
    // 
    
    Ok(())
}