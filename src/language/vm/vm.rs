use crate::language::tokens::*;
use crate::utils::stack::Stack;

type Instruction = u8;
type Bytecode = Vec<Instruction>;

type Value = usize;

pub struct NxVirtualMachine {
    stack: Stack<Value>,
}

impl NxVirtualMachine {
    pub fn new() -> Self {
        NxVirtualMachine {
            stack: Stack::new(),
        }
    }
    
    fn execute() {
        
    }
    
    fn current(&self) -> Option<&Value> {
        self.stack.peek()
    }
}
