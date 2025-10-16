pub mod tokens;
pub mod datatypes;
pub mod expressions;
pub mod errors;
pub mod scopes;

pub fn interpret(source: String) -> Result<(), errors::LangError> {
    let mut program = tokens::Program::new(source);
    program.tokenize();
    program.begin()
}

pub fn tokenize(source: String) -> Result<(), errors::LangError> {
    let mut program = tokens::Program::new(source);
    program.tokenize();
    println!("{}", program);
    Ok(())
}