use crate::language::{datatypes::DataType, errors::LangError};
use std::collections::HashMap;

pub type LowerOrderFunction = fn(&[DataType]) -> Result<DataType, LangError>;

pub struct FunctionRegistry {
    functions: HashMap<String, LowerOrderFunction>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        FunctionRegistry {
            functions: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, func: LowerOrderFunction) {
        self.functions.insert(name.to_string(), func);
    }

    pub fn get(&self, name: &str) -> Option<&LowerOrderFunction> {
        self.functions.get(name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    pub fn call(&self, name: &str, args: &[DataType]) -> Result<DataType, LangError> {
        if let Some(func) = self.get(name) {
            func(args)
        } else {
            Err(LangError::new(format!("Native function '{}' not found", name)))
        }
    }

    pub fn extend(&mut self, other: FunctionRegistry) {
        for (name, func) in other.functions {
            self.register(&name, func);
        }
    }
}

// Initialize all standard library functions
/*pub fn create_std_registry() -> NativeFunctionRegistry {
    let mut registry = NativeFunctionRegistry::new();
    
    // Register all std functions
    // crate::std_lib::register_std_functions(&mut registry);
    
    registry
}*/