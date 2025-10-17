use std::collections::HashMap;
use crate::language::{binder::FunctionRegistry, datatypes::DataType, errors::LangError, expressions::Expression};

#[derive(Clone)]
pub struct Scope {
    variables: HashMap<String, DataType>,
}

pub struct ScopeStack {
    scopes: Vec<Scope>,
    registry: Option<*const FunctionRegistry>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack {
            scopes: vec![Scope::new()],
            registry: None,
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() >= 1 {
            self.scopes.pop();
        }
    }

    pub fn set_native_registry(&mut self, registry: &FunctionRegistry) {
        self.registry = Some(registry as *const FunctionRegistry);
    }

    pub fn get_native_registry(&self) -> Option<&FunctionRegistry> {
        self.registry.map(|ptr| unsafe { &*ptr })
    }

    pub fn define_function(&mut self, fn_name: String, params: Vec<String>, body: Box<Expression>) {
        self.scopes.last_mut().unwrap().variables.insert(fn_name, DataType::Function(params, (*body).clone()));
    }

    pub fn declare(&mut self, var_name: String, value: DataType) {
        self.scopes.last_mut().unwrap().variables.insert(var_name, value);
    }

    pub fn set(&mut self, var_name: &str, value: DataType) -> Result<(), LangError> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.variables.contains_key(var_name) {
                scope.variables.insert(var_name.to_string(), value);
                return Ok(());
            }
        }
        Err(LangError::new(format!("Variable '{}' is not declared", var_name)))
    }

    pub fn get(&mut self, var_name: &str) -> Option<&DataType> {
        for scope in self.scopes.iter().rev() {
            if scope.variables.contains_key(var_name) {
                return Some(scope.variables.get(var_name).unwrap());
            }
        }
        None
    }
}