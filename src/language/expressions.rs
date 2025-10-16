use crate::language::{datatypes::{DataType, DataTypeType}, errors::LangError, scopes::ScopeStack};

pub enum Expression {
    Atom(String),
    Operation(String, Vec<Expression>),
    Declaration(String),
    If(Box<Expression>, Box<Expression>, Option<Box<Expression>>), 
    Block(Vec<Expression>),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            }
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
        !self.is_block() //&& !self.is_conditional()
    }

    pub fn eval(&self, scopes: &mut ScopeStack) -> Result<DataType, LangError> {
        match self {
            Expression::Declaration(decl) => return Err(
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

                Ok(DataType::String(val.clone()))
                /*
                In case its something else(?)
                if val.starts_with("\"") && val.ends_with("\"") {
                    let mut cloned_str = val.clone();
                    cloned_str.remove(0);
                    cloned_str.remove(cloned_str.len()-1);

                    return Ok(DataType::String(cloned_str));
                } */

                /*return Err(
                    LangError::new(format!("Invalid evaluation of atom:  \x1b[1;32m\"{}\"\x1b[0m", val))
                ) */
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
                                "==" => Ok(DataType::Bool(lhs.as_float() == rhs.as_float())),
                                ">" => {
                                    if lhs.get_type() == DataTypeType::Float && rhs.get_type() == DataTypeType::Float {
                                        return Ok(DataType::Bool(lhs.as_float() > rhs.as_float()));
                                    }

                                    return Err(
                                        LangError::new(format!("Cannot compare: \x1b[1;32m\"{} > {}\"\x1b[0m", lhs, rhs))
                                    )
                                },
                                /*"!=" => if lhs != rhs { 1.0 } else { 0.0 },
                                "<" => if lhs < rhs { 1.0 } else { 0.0 },
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
                        if cond.is_truthy() {
                            then_body.eval(scopes)
                        } else if let Some(else_expr) = else_body {
                            else_expr.eval(scopes)
                        } else {
                            Ok(DataType::Float(0.0))
                        }
                    },
                    Err(err) => return Err(err),
                }
            },
            Expression::Block(expressions) => {
                scopes.push_scope();
                
                let mut result = DataType::EndOfBlock;
                for expr in expressions {
                    if let Some((var_name, expr_tree, is_declaration)) = expr.is_assign() {
                        let value = expr_tree.eval(scopes)?;
                        
                        if is_declaration {
                            scopes.declare(var_name, value.clone());
                            result = value;
                        } else {
                            scopes.set(&var_name, value.clone())?;
                            result = value;
                        }
                    } else {
                        match expr.eval(scopes) {
                            Ok(val) => result = val,
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