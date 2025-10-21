use crate::language::binder::FunctionRegistry;
use crate::language::errors::LangError;
use crate::language::expressions::*;
use crate::language::scopes::ScopeStack;
use crate::language::stdlib;

static LINE_END_TOKEN: &str = ";";

#[derive(Clone, PartialEq, Debug)]
pub enum SplitTokenType {
    CharToken,
    NumToken,
    StrToken,
    SplitToken,
    OperationToken,
    EndExpressionToken,
}


#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    // Arithmetic
    OperationToken(String),

    // Program creation
    LetToken(String), // variable declaration
    MutableToken(String), // make variable mutable
    IdentifierToken(String), // variable name
    ScopeBeginToken(String), // {
    ScopeEndToken(String), // }
    OpenParenthesisToken(String), // (
    CloseParenthesisToken(String), // )
    EndExpressionToken(String), // ;

    // Types
    BoolToken(String),
    NumericToken(String),
    StringToken(String),

    // Logic
    IfToken(String),
    ElseToken(String),
    ReturnToken(String),

    EqualToken(String), // =
    CompareToken(String),
    NotEqualToken(String), // !=
    GreaterToken(String), // >
    LessToken(String), // <
    GreaterEqualToken(String), // >=
    LessEqualToken(String), // <=
    AndToken(String), // &&
    OrToken(String), // ||        
    NamespaceAccessToken(String), // ::

    // Loops
    WhileToken(String),
    ForToken(String),
    LoopToken(String),
    BreakToken(String),
    ContinueToken(String),

    // Functions
    FunctionToken(String),
    PublicToken(String),
    PrivateToken(String),
    ProtectedToken(String),

    // non-registered
    InvalidToken(String),
    FinishLine(String),
    SplitToken(String),
    
    // other
    EofToken,
}

#[derive(Clone)]
pub struct SplitToken {
    pub token_type: SplitTokenType,
    pub value: String,
}

pub struct Program {
    pub source: String,
    pub tokens: Vec<Token>,
    pub scopes: ScopeStack,
    pub registry: FunctionRegistry,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::OperationToken(val) => write!(f, "\x1b[0;31mToken<\x1b[1;31mOperator, \'{}\'\x1b[0;31m>\x1b[0m", val),
            Token::LetToken(val) => write!(f, "\x1b[0;32mToken\x1b[1;32m<Decl, {}>\x1b[0m", val),
            Token::IdentifierToken(val) => write!(f, "\x1b[0;33mToken\x1b[1;33m<Identifier, \"{}\">\x1b[0m", val),
            Token::NumericToken(val) => write!(f, "\x1b[0;33mToken\x1b[1;33m<Number, {}>\x1b[0m", val),
            Token::EndExpressionToken(val) => write!(f, "\x1b[0;34mToken\x1b[1;34m<EndExpression, {}>\x1b[0m", val),
            Token::ScopeBeginToken(val) => write!(f, "\x1b[0;35mToken\x1b[1;35m<ScopeBegin, {}>\x1b[0m", val),
            Token::ScopeEndToken(val) => write!(f, "\x1b[0;35mToken\x1b[1;35m<ScopeEnd, {}>\x1b[0m", val),
            Token::FunctionToken(val) => write!(f, "\x1b[0;31mToken\x1b[1;31m<DeclFunction, {}>\x1b[0m", val),
            Token::StringToken(val) => write!(f, "\x1b[0;32mToken\x1b[1;32m<String, {}>\x1b[0m", val),
            Token::ReturnToken(val) => write!(f, "\x1b[0;31mToken\x1b[1;31m<ReturnVal, {}>\x1b[0m", val),


            t => write!(f, "{:?}", t),
        }
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.tokens.iter().rev() {
            write!(f, "{}\n", token)?;
        }
        Ok(())
    }
}


pub fn operator_binding_power(token: &str) -> (f32, f32) {
    match token {
        "=" => (0.1, 0.2),
        "||" => (0.3, 0.4),
        "&&" => (0.5, 0.6),
        "==" | "!=" => (0.7, 0.8),
        "<" | ">" | "<=" | ">=" => (0.9, 1.0),
        "+" | "-" => (1.0, 1.1),
        "*" | "/" => (2.0, 2.1),
        "." | "[" => (4.0, 4.1),
        _ => panic!("Invalid operator: {}", token),
    }
}

fn parse_function_signature(signature: &str) -> Result<(String, Vec<String>), LangError> {
    // Parse "name(param1,param2)" into ("name", ["param1", "param2"])
    if let Some(paren_pos) = signature.find('(') {
        let fn_name = signature[..paren_pos].to_string();
        let params_str = &signature[paren_pos+1..signature.len()-1]; // Remove ( and )
        
        let params: Vec<String> = if params_str.is_empty() {
            vec![]
        } else {
            params_str.split(',')
                .map(|s| s.trim().to_string())
                .collect()
        };
        
        Ok((fn_name, params))
    } else {
        Err(LangError::new(format!("Invalid function signature: {}", signature)))
    }
}

fn is_function_call(identifier: &str) -> bool {
    identifier.contains('(') && identifier.ends_with(')')
}

fn parse_function_call_with_program(call_str: &str) -> Result<(String, Vec<Expression>), LangError> {
    if let Some(paren_pos) = call_str.find('(') {
        let fn_name = call_str[..paren_pos].to_string();
        let args_str = &call_str[paren_pos+1..call_str.len()-1]; // Remove ( and )
        
        if args_str.is_empty() {
            return Ok((fn_name, vec![]));
        }
        
        let arg_strings = split_arguments(args_str);
        
        let mut arg_expressions = Vec::new();
        for arg_str in arg_strings {
            let mut mini_program = Program::new();
            mini_program.tokenize(arg_str.trim().to_string());
            let expr = mini_program.parse_expression(0.0)?;
            arg_expressions.push(expr);
        }
        
        Ok((fn_name, arg_expressions))
    } else {
        Err(LangError::new(format!("Invalid function call: {}", call_str)))
    }
}

fn split_arguments(args_str: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut paren_depth = 0;
    let mut in_string = false;
    
    for ch in args_str.chars() {
        match ch {
            '"' => {
                in_string = !in_string;
                current_arg.push(ch);
            }
            '(' if !in_string => {
                paren_depth += 1;
                current_arg.push(ch);
            }
            ')' if !in_string => {
                paren_depth -= 1;
                current_arg.push(ch);
            }
            ',' if !in_string && paren_depth == 0 => {
                args.push(current_arg.trim().to_string());
                current_arg.clear();
            }
            _ => {
                current_arg.push(ch);
            }
        }
    }
    
    if !current_arg.is_empty() {
        args.push(current_arg.trim().to_string());
    }
    
    args
}


impl Token {
    pub fn is_declaration(&self) -> bool {
        matches!(self, Token::LetToken(_))
    }

    pub fn is_expression_end(&self) -> bool {
        matches!(self, Token::EndExpressionToken(_) | Token::ScopeEndToken(_))
    }
}

impl Program {
    pub fn new() -> Self {
        Program {
            source: "".to_string(),
            tokens: vec![],
            scopes: ScopeStack::new(),
            registry: FunctionRegistry::new(),
        }
    }

    pub fn tokenize(&mut self, source: String) {
        let tokens: Vec<SplitToken>  =  source
            .chars()
            .map(|token_char| match token_char {
                '0'..='9' => SplitToken {
                    token_type: SplitTokenType::NumToken,
                    value: token_char.to_string(),
                },
                'a'..='z' | 'A'..='Z' | '_' | ',' => SplitToken {
                    token_type: SplitTokenType::CharToken,
                    value: token_char.to_string(),
                },
                ' ' | '\t' | '\n' | '\r' => SplitToken {
                    token_type: SplitTokenType::SplitToken,
                    value: token_char.to_string(),
                },
                '\"' => SplitToken {
                    token_type: SplitTokenType::StrToken,
                    value: token_char.to_string(),
                },
                ';' => SplitToken {
                    token_type: SplitTokenType::EndExpressionToken,
                    value: token_char.to_string(),
                },
                _ => SplitToken {
                    token_type: SplitTokenType::OperationToken,
                    value: token_char.to_string(),
                },
            }).collect();

        self.process_tokens(tokens);
    }

    fn process_tokens(&mut self, tokens: Vec<SplitToken>) {
        let mut new_tokens: Vec<Token> = Vec::new();
        
        use crate::language::tokenizer::tokenize;
        tokenize(&mut new_tokens, &tokens);
        
        new_tokens.reverse();

        self.tokens = new_tokens;
    }
    
    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::EofToken)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::EofToken)
    }

    pub fn begin(&mut self) -> Result<(), LangError> {
        stdlib::register_std_functions(&mut self.registry);
        self.scopes.set_native_registry(&self.registry);

        loop {
            let next_token = self.peek();
            if next_token == Token::EofToken {
                break;
            } else if next_token.is_expression_end() {
                self.next();
                continue;
            }

            match self.parse_expression(0.0) {
                Ok(expr) => {
                    if let Expression::FunctionDeclaration(fn_name, params, body) = expr {
                        self.scopes.define_function(fn_name, params, body);
                    } else if let Some((var_name, expr_tree, is_declaration)) = expr.is_assign() {
                        let wrapped = expr_tree.eval(&mut self.scopes);
                        
                        match wrapped {
                            Ok(value) => {
                                if is_declaration {
                                    self.scopes.declare(var_name, value);
                                } else if let Err(err) = self.scopes.set(&var_name, value) {
                                    return Err(err);
                                }
                            },

                            Err(err) => return Err(err)
                        }
                    } else {
                        match expr.eval(&mut self.scopes) {
                            Ok(value) => {
                                if expr.evaluate_print() {
                                    println!("{}", value)
                                }
                            },
                            Err(err) => return Err(err),
                        };
                    };
                },
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    pub fn parse_block(&mut self) -> Expression {
        let mut expressions = Vec::new();
        
        loop {
            match self.peek() {
                Token::ScopeEndToken(_) | Token::EofToken => break,
                Token::EndExpressionToken(_) => {
                    self.next();
                    continue;
                }
                _ => {
                    expressions.push(self.parse_expression(0.0).unwrap());
                }
            }
        }
        
        Expression::Block(expressions)
    }

    pub fn parse_expression(&mut self, min_bp: f32) -> Result<Expression, LangError> {
        let mut lvalue = match self.next() {
            Token::EndExpressionToken(_) => {
                return self.parse_expression(min_bp);
            }
            Token::LetToken(_) => {
                match self.next() {
                    Token::IdentifierToken(var_name) => Expression::Declaration(var_name),
                    t => return Err(LangError::new(format!("Expected identifier after 'let', got: {:?}", t))),
                }
            },
            Token::FunctionToken(_) => {
                match self.next() {
                    Token::IdentifierToken(fn_signature) => {
                        let (fn_name, params) = parse_function_signature(&fn_signature)?;
                        
                        assert_eq!(self.next(), Token::ScopeBeginToken("{".to_string()));
                        let body = self.parse_block();
                        assert_eq!(self.next(), Token::ScopeEndToken("}".to_string()));
                        
                        Expression::FunctionDeclaration(fn_name, params, Box::new(body))
                    }
                    t => return Err(LangError::new(format!("Expected function signature after 'function', got: {:?}", t))),
                }
            },
            Token::IfToken(_) => {
                let condition = self.parse_expression(0.0).unwrap();
                
                assert_eq!(self.next(), Token::ScopeBeginToken("{".to_string()));
                let then_body = self.parse_block();
                assert_eq!(self.next(), Token::ScopeEndToken("}".to_string()));
                
                let else_body = Option::Some(Box::new(Expression::Block(vec![])));
                
                Expression::If(Box::new(condition), Box::new(then_body), else_body)
            },
            Token::ScopeBeginToken(_) => {
                let block = self.parse_block();
                assert_eq!(self.next(), Token::ScopeEndToken("}".to_string()));
                block
            },
            Token::BoolToken(val) => Expression::Atom(val),
            Token::StringToken(val) => Expression::Atom(val),
            Token::IdentifierToken(var_name) => {
                if is_function_call(&var_name) {
                    let (fn_name, arg_expressions) = parse_function_call_with_program(&var_name)?;
                    Expression::FunctionCall(fn_name, Box::new(arg_expressions))
                } else {
                    Expression::Atom(var_name)
                }
            },
            Token::NumericToken(var_name) => Expression::Atom(var_name),
            Token::OpenParenthesisToken(_) => {
                let last_expr = self.parse_expression(0.0);
                assert_eq!(self.next(), Token::CloseParenthesisToken(")".to_string()));
                last_expr.unwrap()
            },
            Token::ReturnToken(_) => {
                let result = self.parse_expression(0.0);
                match result {
                    Ok(return_val) => Expression::Return(Box::new(return_val)),
                    Err(err) => return Err(err),
                }
            },
            
            Token::ForToken(_) => {
                // Expect: for i = start, end [, step]
                
                // Parse variable name
                let var_name = match self.next() {
                    Token::IdentifierToken(name) => name,
                    t => return Err(LangError::new(format!("Expected variable name after 'for', got: {:?}", t))),
                };
                
                match self.next() {
                    Token::EqualToken(op) | Token::OperationToken(op) if op == "=" => {},
                    t => return Err(LangError::new(format!("Expected '=' after for variable, got: {:?}", t))),
                }
                
                let start = self.parse_expression(0.0)?;
                
                match self.next() {
                    Token::IdentifierToken(ref s) if s == "," => {},
                    Token::OperationToken(ref s) if s == "," => {},
                    t => return Err(LangError::new(format!("Expected ',' after for start value, got: {:?}", t))),
                }
                
                let end = self.parse_expression(0.0)?;
                
                let step = match self.peek() {
                    Token::IdentifierToken(ref s) | Token::OperationToken(ref s) => {
                        if s == "," {
                            None
                        } else {
                            self.next();
                            Some(Box::new(self.parse_expression(0.0)?))
                        }
                    },
                    _ => None,
                };
                
                match self.next() {
                    Token::ScopeBeginToken(_) => {},
                    t => return Err(LangError::new(format!("Expected '{{' after for parameters, got: {:?}", t))),
                }
                
                let body = self.parse_block();
                match self.next() {
                    Token::ScopeEndToken(_) => {},
                    t => return Err(LangError::new(format!("Expected '}}' after for body, got: {:?}", t))),
                }
                
                Expression::ForLoop(var_name, Box::new(start), Box::new(end), step, Box::new(body))
            },
            Token::WhileToken(_) => {
                let condition = self.parse_expression(0.0)?;
                match self.next() {
                    Token::ScopeBeginToken(_) => {},
                    t => return Err(LangError::new(format!("Expected '{{' after while condition, got: {:?}", t))),
                }
                
                let body = self.parse_block();
                match self.next() {
                    Token::ScopeEndToken(_) => {},
                    t => return Err(LangError::new(format!("Expected '}}' after while body, got: {:?}", t))),
                }
                
                Expression::WhileLoop(Box::new(condition), Box::new(body))
            },
            
            Token::LoopToken(_) => {
                match self.next() {
                    Token::ScopeBeginToken(_) => {},
                    t => return Err(LangError::new(format!("Expected '{{' after loop, got: {:?}", t))),
                }
                
                let body = self.parse_block();
                match self.next() {
                    Token::ScopeEndToken(_) => {},
                    t => return Err(LangError::new(format!("Expected '}}' after loop body, got: {:?}", t))),
                }
                
                Expression::InfiniteLoop(Box::new(body))
            },
            
            Token::BreakToken(_) => Expression::Break,
            
            Token::ContinueToken(_) => Expression::Continue,
            t => return Err(
                LangError::new(format!("[Tokens]: Unknown reference to \x1b[1;32m\"{:?}\"\x1b[0m", t))
            ),
        };

        loop {
            let op = match self.peek() {
                Token::EofToken | Token::EndExpressionToken(_) => break,
                Token::CloseParenthesisToken(_) => break,
                Token::OperationToken(opv) 
                | Token::EqualToken(opv)
                | Token::CompareToken(opv) => opv,
                _ => break,
            };

            let (l_bp, r_bp) = operator_binding_power(&op);
            if l_bp < min_bp {
                break;
            }

            self.next();
            match self.parse_expression(r_bp) {
                Ok(rvalue) => lvalue = Expression::Operation(op, vec![lvalue, rvalue]),
                Err(err) => return Err(err),
            }
        };

        Ok(lvalue)
    }

    pub fn is_token_ending(token: &SplitToken) -> bool {
        token.value == LINE_END_TOKEN || token.value == " "
    }
}