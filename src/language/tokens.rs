use crate::language::errors::LangError;
use crate::language::expressions::*;
use crate::language::scopes::ScopeStack;

static LINE_END_TOKEN: &str = ";";

#[derive(Clone, PartialEq)]
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

            val => write!(f, "\x1b[0;37mToken<Operator, {}>\x1b[0m", val),
        }
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.tokens.iter() {
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
        _ => panic!("Invalid operator: {}", token),
    }
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
    pub fn new(source: String) -> Self {
        Program {
            source,
            tokens: vec![],
            scopes: ScopeStack::new(),
        }
    }

    pub fn tokenize(&mut self) {
        let source = self.source.clone();
        let tokens: Vec<SplitToken>  =  source
            .chars()
            .map(|token_char| match token_char {
                '0'..='9' => SplitToken {
                    token_type: SplitTokenType::NumToken,
                    value: token_char.to_string(),
                },
                'a'..='z' | 'A'..='Z' | '_' => SplitToken {
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

    fn process_name_token(&self, token_str: &str) -> Token {
        match token_str {
            "let" | "local" | "var" => Token::LetToken(token_str.to_string()),
            "if" => Token::IfToken(token_str.to_string()),
            "else" => Token::ElseToken(token_str.to_string()),
            "while" => Token::WhileToken(token_str.to_string()),
            "for" => Token::ForToken(token_str.to_string()),
            "function" => Token::FunctionToken(token_str.to_string()),
            "public" => Token::PublicToken(token_str.to_string()),
            "private" => Token::PrivateToken(token_str.to_string()),
            "protected" => Token::ProtectedToken(token_str.to_string()),
            "be" => Token::EqualToken(token_str.to_string()),
            "true" | "false" => Token::BoolToken(token_str.to_string()),
            _ => Token::IdentifierToken(token_str.to_string()),
        }
    }

    fn process_tokens(&mut self, tokens: Vec<SplitToken>) {
        let mut new_tokens: Vec<Token> = Vec::new();
        let mut cur_idx = 0;

        while cur_idx < tokens.len() {
            let current_token = &tokens[cur_idx];
            if current_token.value == " " {
                cur_idx += 1;
                continue;
            }

            match current_token.token_type {
                SplitTokenType::CharToken => {
                    let mut base_str = String::new();
                    let mut next_token_idx = cur_idx;
                    
                    while next_token_idx < tokens.len() {
                        let next_token: &SplitToken = &tokens[next_token_idx];
                        if (next_token.token_type != SplitTokenType::CharToken && next_token.token_type != SplitTokenType::NumToken) || next_token.value == " " {
                            break;
                        }
                        base_str.push_str(&next_token.value);
                        next_token_idx += 1;
                    }
                    
                    let current_token = self.process_name_token(&base_str);

                    new_tokens.push(current_token);
                    
                    cur_idx = next_token_idx;    
                },
                SplitTokenType::StrToken => {
                    let mut base_str = String::new();
                    let mut next_token_idx = cur_idx;
                    
                    while next_token_idx < tokens.len() {
                        let next_token: &SplitToken = &tokens[next_token_idx];
                        base_str.push_str(&next_token.value);
                        next_token_idx+=1;

                        if next_token.token_type == SplitTokenType::StrToken && next_token.value == "\"" && (next_token_idx-1) > cur_idx {
                            break;
                        }
                    }

                    base_str.remove(0); // initial "
                    base_str.remove(base_str.len() - 1);

                    let current_token = Token::StringToken(base_str);
                    new_tokens.push(current_token);
                    
                    cur_idx = next_token_idx; 
                },
                SplitTokenType::NumToken => {
                    let mut base_str = String::new();
                    let mut next_token_idx = cur_idx;
                    
                    while next_token_idx < tokens.len() {
                        let next_token = &tokens[next_token_idx];
                        if next_token.token_type != SplitTokenType::NumToken || Program::is_token_ending(next_token) {
                            break;
                        }
                        base_str.push_str(&next_token.value);
                        next_token_idx += 1;
                    }
                    
                    new_tokens.push(Token::NumericToken(base_str));
                    
                    cur_idx = next_token_idx; 
                },
                SplitTokenType::OperationToken | SplitTokenType::EndExpressionToken => {
                    let mut added_token = match current_token.value.as_str() {
                        "{" => Token::ScopeBeginToken(current_token.value.clone()),
                        "}" => Token::ScopeEndToken(current_token.value.clone()),
                        "(" => Token::OpenParenthesisToken(current_token.value.clone()),
                        ")" => Token::CloseParenthesisToken(current_token.value.clone()),
                        ";" => Token::EndExpressionToken(current_token.value.clone()),
                        _ => Token::OperationToken(current_token.value.clone()),
                    };

                    let mut base_str = String::new();
                    base_str.push_str(&current_token.value);

                    if cur_idx + 1 < tokens.len() {
                        let next_token = &tokens[cur_idx + 1];

                        if next_token.token_type == SplitTokenType::OperationToken && ( next_token.value == "=" || current_token.value == "=" ) {
                            base_str.push_str(&next_token.value);
                            cur_idx += 1;

                            added_token = match base_str.as_str() {
                                "==" => Token::CompareToken(base_str),
                                "!=" => Token::NotEqualToken(base_str),
                                ">=" => Token::GreaterEqualToken(base_str),
                                "<=" => Token::LessEqualToken(base_str),
                                _ => added_token,
                            }
                        }
                    }

                    new_tokens.push(added_token);

                    cur_idx += 1;
                }
                _ => {
                    cur_idx += 1;
                }
            }
        }

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
                    if let Some((var_name, expr_tree, is_declaration)) = expr.is_assign() {
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
            Token::IfToken(_) => {
                let condition = self.parse_expression(0.0).unwrap();
                
                assert_eq!(self.next(), Token::ScopeBeginToken("{".to_string()));
                let then_body = self.parse_block();
                assert_eq!(self.next(), Token::ScopeEndToken("}".to_string()));
                
                let else_body = Option::Some(Box::new(Expression::Block(vec![])));
                
                Expression::If(Box::new(condition), Box::new(then_body), else_body)
            },
            Token::ScopeBeginToken(_) => Expression::Block(vec![]),
            Token::BoolToken(val) => Expression::Atom(val),
            Token::StringToken(val) => Expression::Atom(val),
            Token::IdentifierToken(var_name) => Expression::Atom(var_name),
            Token::NumericToken(var_name) => Expression::Atom(var_name),
            Token::OpenParenthesisToken(_) => {
                let last_expr = self.parse_expression(0.0);
                assert_eq!(self.next(), Token::CloseParenthesisToken(")".to_string()));
                last_expr.unwrap()
            },
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