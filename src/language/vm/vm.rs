// use crate::language::expressions::Expression;
use crate::language::tokens::*;
use crate::utils::stack::Stack;

pub enum Instruction {
    NoOp,
    IntImmediate,
}
pub type Bytecode = Vec<Instruction>;
pub type Value = usize;


pub struct NxVirtualMachine {
    pub stack: Stack<Value>,
    pub pointer: usize,
}

impl NxVirtualMachine {
    pub fn new() -> Self {
        NxVirtualMachine {
            pointer: 0,
            stack: Stack::new(),
        }
    }
    
    fn _execute() {
        
    }
    
    fn _current(&self) -> Option<&Value> {
        self.stack.peek()
    }
}

pub struct BytecodeBuilder {}

impl BytecodeBuilder {
    pub fn new() -> Self {
        BytecodeBuilder {}
    }
    
    fn _emit_instruction(&self) {
        
    }
    
    pub fn build(&self) -> Bytecode {
        vec![]
    }
    
    pub fn generate(&self, _tokens: Vec<&Token>) {
        
    }
}

pub fn generate_bytecode(tokens: Vec<&Token>) -> Result<Bytecode, String> {
    let bytecode_builder = BytecodeBuilder::new();
    bytecode_builder.generate(tokens);

    Ok(bytecode_builder.build())    
}

