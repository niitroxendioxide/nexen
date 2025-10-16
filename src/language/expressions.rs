use std::collections::HashMap;
use crate::language::{datatypes::DataType, errors::LangError};

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

    pub fn eval(&self, variables: &HashMap<String, DataType>) -> Result<DataType, LangError> {
        match self {
            Expression::Declaration(_) => panic!("Cannot evaluate a declaration"),
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

                if variables.contains_key(val) {
                    return Ok(variables[val]);
                }

                return Err(
                    LangError::new(format!("Unknown reference to \x1b[1;32m\"{}\"\x1b[0m", val))
                )
            },
            Expression::Operation(op, tree) => {
                match tree.first().unwrap().eval(variables) {
                    Ok(lhs) => match tree.last().unwrap().eval(variables) {
                        Ok(rhs) => {
                            match op.as_str() {
                                "+" => return Ok(DataType::Float(lhs.as_float() + rhs.as_float())),
                                "-" => return Ok(DataType::Float(lhs.as_float() - rhs.as_float())),
                                "*" => return Ok(DataType::Float(lhs.as_float() * rhs.as_float())),
                                "/" => return Ok(DataType::Float(lhs.as_float() / rhs.as_float())),
                                "=" => return Ok(lhs),
                                "==" => Ok(DataType::Bool(lhs.as_float() == rhs.as_float())),
                                /*"!=" => if lhs != rhs { 1.0 } else { 0.0 },
                                ">" => if lhs > rhs { 1.0 } else { 0.0 },
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
                match condition.eval(variables) {
                    Ok(cond) => {
                        if cond.is_truthy() {
                            then_body.eval(variables)
                        } else if let Some(else_expr) = else_body {
                            else_expr.eval(variables)
                        } else {
                            Ok(DataType::Float(0.0))
                        }
                    },
                    Err(err) => return Err(err),
                }
            },
            Expression::Block(expressions) => {
                let mut result = DataType::Float(0.0);
                for expr in expressions {
                    result = expr.eval(variables).unwrap();
                }
                Ok(result)
            }
        }
    }
}