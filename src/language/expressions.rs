use crate::language::{datatypes::{DataType, DataTypeType}, errors::LangError, scopes::ScopeStack};

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Atom(String),
    Operation(String, Vec<Expression>),
    Declaration(String),

    // fn
    FunctionCall(String, Box<Vec<Expression>>),
    FunctionDeclaration(String, Vec<String>, Box<Expression>),
    Return(Box<Expression>),
    
    // conditionals
    If(Box<Expression>, Box<Expression>, Option<Box<Expression>>), 
    Block(Vec<Expression>),
    ForLoop(String, Box<Expression>, Box<Expression>, Option<Box<Expression>>, Box<Expression>),
    // WhileLoop(Box<Expression>, Box<Expression>),  // (condition, body)
    // InfiniteLoop(Box<Expression>),  // (body)
    // Break,  // break statement
    // Continue,  // continue statement
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::FunctionDeclaration(name, params, body) => {
                let mut param_str = String::new();
                for param in params {
                    param_str.push_str(&format!("{}, ", param));
                }

                write!(f, "fn<{}({})> {{ {} }}", name, param_str, body)
            },
            Expression::FunctionCall(fn_name, _) => {

                write!(f, "fn_call<{}>", fn_name)
            },
            Expression::Declaration(val) => write!(f, "decl<{}>", val),
            Expression::Atom(val) => write!(f, "{}", val),
            Expression::Operation(op, tree) => {
                write!(f, "({}", op)?;
                for expr in tree {
                    write!(f, " {}", expr)?;
                }
                write!(f, ")")
            },
            Expression::If(condition, then, _else_) => {
                write!(f, "if {} then {} else {}", condition, then, "")
            },
            Expression::Block(tree) => {
                write!(f, "{{ ")?;
                for expr in tree {
                    write!(f, "{} ", expr)?;
                }
                write!(f, "}}")
            },
            t=>write!(f, "{:?}", t),
        }
    }
}

impl Expression {
    pub fn is_assign(&self) -> Option<(String, &Expression, bool)> {
        match self {
            Expression::Operation(op, tree) => {
                if op == "=" {
                    let (var_name, is_declaration) = match tree.first().unwrap() {
                        Expression::Declaration(var_name) => (var_name.clone(), true),
                        Expression::Atom(var_name) => {
                            if var_name.chars().next().map(|x| matches!(x, 'a'..='z' | 'A'..='Z')).unwrap_or(false) {
                                (var_name.clone(), false)
                            } else {
                                panic!("Invalid variable name: {}", var_name);
                            }
                        },
                        _ => unreachable!(),
                    };

                    return Some((var_name, tree.last().unwrap(), is_declaration));
                }

                return None;
            },
            _ => None,
        }
    }

    pub fn is_block(&self) -> bool {
        matches!(self, Expression::Block(_))
    }

    pub fn is_conditional(&self) -> bool {
        matches!(self, Expression::If(..))
    }

    pub fn evaluate_print(&self) -> bool {
        false // !self.is_block() //&& !self.is_conditional()
    }

    pub fn eval(&self, scopes: &mut ScopeStack) -> Result<DataType, LangError> {
        match self {
            Expression::Return(expr) => {
                let evaluated = expr.eval(scopes);
                match evaluated {
                    Ok(value) => Ok(DataType::Return(Box::new(value))),
                    Err(err) => return Err(err),
                }
            },
            Expression::ForLoop(var_name, start_expr, end_expr, step_expr, body) => {
                // Evaluate start, end, and step
                let start_val = start_expr.eval(scopes)?.as_float();
                let end_val = end_expr.eval(scopes)?.as_float();
                let step_val = if let Some(step) = step_expr {
                    step.eval(scopes)?.as_float()
                } else {
                    1.0 // Default step is 1
                };
                
                // Validate step
                if step_val == 0.0 {
                    return Err(LangError::new("For loop step cannot be zero".to_string()));
                }
                
                // Create a new scope for the loop
                scopes.push_scope();
                
                let mut result = DataType::EndOfBlock;
                let mut current = start_val;
                
                // Determine loop direction
                let ascending = step_val > 0.0;
                
                loop {
                    // Check loop condition based on direction
                    if ascending && current > end_val {
                        break;
                    }
                    if !ascending && current < end_val {
                        break;
                    }
                    
                    // Set loop variable
                    scopes.set_or_declare(var_name.clone(), DataType::Float(current));
                    
                    // Execute body
                    match body.eval(scopes) {
                        Ok(val) => {
                            // Check for early return
                            if matches!(val, DataType::Return(_)) {
                                scopes.pop_scope();
                                return Ok(val);
                            }
                            result = val;
                        },
                        Err(err) => {
                            scopes.pop_scope();
                            return Err(err);
                        }
                    }
                    
                    // Increment/decrement
                    current += step_val;
                }
                
                scopes.pop_scope();
                Ok(result)
            },
            Expression::FunctionCall(fn_name, args) => {
                let mut arg_values = Vec::new();
                for arg in args.iter() {
                    match arg.eval(scopes) {
                        Ok(val) => arg_values.push(val),
                        Err(err) => return Err(err),
                    }
                }

                if let Some(scope_registry) = scopes.get_native_registry() {                    
                    if scope_registry.has(&fn_name) {
                        return scope_registry.call(&fn_name, &arg_values);
                    }
                }

                let (params, body) = match scopes.get(&fn_name) {
                    Some(fn_data) => {
                        match fn_data {
                            DataType::Function(params, body) => {
                                if params.len() != args.len() {
                                    return Err(LangError::new(format!(
                                        "Function '{}' expects {} arguments, got {}",
                                        fn_name, params.len(), args.len()
                                    )));
                                }
                                (params.clone(), body.clone())
                            }
                            _ => return Err(LangError::new(format!("'{}' is not a function", fn_name)))
                        }
                    },
                    None => return Err(LangError::new(format!("Function '{}' is not defined", fn_name))),
                };
                
                scopes.push_scope();
                
                for (param_name, arg_value) in params.iter().zip(arg_values.iter()) {
                    scopes.declare(param_name.clone(), arg_value.clone());
                }
                
                // eval body & leave scope
                let result = body.eval(scopes);
                scopes.pop_scope();
                
                match result {
                    Ok(DataType::Return(inner) )=> Ok(*inner),
                    Ok(other) => Ok(other),
                    Err(e) => Err(e),
                }
            },
            Expression::Declaration(decl) | Expression::FunctionDeclaration(decl, ..)  => return Err(
                LangError::new(format!("Cannot evaluate declaration: {}", decl))
            ),
            Expression::Atom(val) => {
                if val == "true" {
                    return Ok(DataType::Bool(true));
                }
                if val == "false" {
                    return Ok(DataType::Bool(false));
                }

                if let Ok(num) = val.parse::<f32>() {
                    return Ok(DataType::Float(num));
                }

                if let Some(value) = scopes.get(val) {
                    return Ok(value.clone());
                }

                if val.starts_with("\"") && val.ends_with("\"") {
                    return Ok(DataType::String(val[1..val.len()-1].to_string()));
                }
                
                Err(LangError::new(format!("Variable '{}' is not defined", val)))
            },
            Expression::Operation(op, tree) => {
                match tree.first().unwrap().eval(scopes) {
                    Ok(lhs) => match tree.last().unwrap().eval(scopes) {
                        Ok(rhs) => {
                            match op.as_str() {
                                "+" => {
                                    if lhs.get_type() == DataTypeType::Float && rhs.get_type() == DataTypeType::Float {
                                        return Ok(DataType::Float(lhs.as_float() + rhs.as_float()));
                                    } else if lhs.get_type() == DataTypeType::String {
                                        return Ok(DataType::String(lhs.as_string() + &rhs.as_string()));
                                    }

                                    let r_str = rhs.as_string();
                                    let l_str = lhs.as_string();

                                    Err(
                                        LangError::new(format!("Invalid evaluation: \x1b[1;32m\"{} {} {}\"\x1b[0m", l_str, op, r_str))
                                    )
                                },
                                "-" => return Ok(DataType::Float(lhs.as_float() - rhs.as_float())),
                                "*" => return Ok(DataType::Float(lhs.as_float() * rhs.as_float())),
                                "/" => return Ok(DataType::Float(lhs.as_float() / rhs.as_float())),
                                "=" => return Ok(lhs),
                                "==" => {
                                    if lhs.get_type() == DataTypeType::String {
                                        return Ok(DataType::Bool(lhs.as_string() == rhs.as_string()));
                                    } else {
                                        return Ok(DataType::Bool(lhs.as_float() == rhs.as_float()));
                                    }
                                }
                                ">" => {
                                    if lhs.get_type() == DataTypeType::Float && rhs.get_type() == DataTypeType::Float {
                                        return Ok(DataType::Bool(lhs.as_float() > rhs.as_float()));
                                    }

                                    return Err(
                                        LangError::new(format!("Cannot compare: \x1b[1;32m\"{} > {}\"\x1b[0m", lhs, rhs))
                                    )
                                },
                                "<" => {
                                    if lhs.get_type() == DataTypeType::Float && rhs.get_type() == DataTypeType::Float {
                                        return Ok(DataType::Bool(lhs.as_float() < rhs.as_float()));
                                    }

                                    return Err(
                                        LangError::new(format!("Cannot compare: \x1b[1;32m\"{} > {}\"\x1b[0m", lhs, rhs))
                                    )
                                },
                                /*"!=" => if lhs != rhs { 1.0 } else { 0.0 },
                                ">=" => if lhs >= rhs { 1.0 } else { 0.0 },
                                "<=" => if lhs <= rhs { 1.0 } else { 0.0 },
                                "&&" => if lhs != 0.0 && rhs != 0.0 { 1.0 } else { 0.0 },
                                "||" => if lhs != 0.0 || rhs != 0.0 { 1.0 } else { 0.0 }, */
                                _ => return Err(
                                    LangError::new(format!("Unsupported operator: {}, lhs: {}, rhs: {}", op, lhs, rhs))
                                ),
                            }
                        },
                        Err(err) => return Err(err),
                    },
                    Err(err) => return Err(err),
                }
            },
            Expression::If(condition, then_body, else_body) => {
                match condition.eval(scopes) {
                    Ok(cond) => {
                        let result = if cond.is_truthy() {
                            then_body.eval(scopes)
                        } else if let Some(else_expr) = else_body {
                            else_expr.eval(scopes)
                        } else {
                            Ok(DataType::Float(0.0))
                        };
                        
                        // Propagate return values
                        match result {
                            Ok(DataType::Return(value)) => {
                                 return Ok(DataType::Return(value));
                            },
                            Ok(other) => return Ok(other),
                            Err(err) => return Err(err),
                        }
                    },
                    Err(err) => return Err(err),
                }
            },
            Expression::Block(expressions) => {
                scopes.push_scope();
                
                let mut result = DataType::EndOfBlock;
                for expr in expressions {
                    if let Expression::FunctionDeclaration(fn_name, params, body) = expr {
                        scopes.define_function(fn_name.clone(), params.clone(), body.clone());
                    } else if let Some((var_name, expr_tree, is_declaration)) = expr.is_assign() {
                        let value = expr_tree.eval(scopes)?;
                        
                        if matches!(value, DataType::Return(_)) {
                            scopes.pop_scope();
                            return Ok(value);
                        }
                        
                        if is_declaration {
                            scopes.declare(var_name, value.clone());
                            result = value;
                        } else {
                            scopes.set(&var_name, value.clone())?;
                            result = value;
                        }
                    } else {
                        match expr.eval(scopes) {
                            Ok(val) => {
                                if matches!(val, DataType::Return(_)) {
                                    scopes.pop_scope();
                                    return Ok(val);
                                }
                                
                                result = val;
                            },
                            Err(err) => {
                                scopes.pop_scope();
                                return Err(err);
                            }
                        }
                    }
                }
                
                scopes.pop_scope();
                Ok(result)
            }
        }
    }
}