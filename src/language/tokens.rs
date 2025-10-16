use std::collections::HashMap;

static LINE_END_TOKEN: &str = ";";

#[derive(Clone, PartialEq)]
pub enum SplitTokenType {
    CharToken,
    NumToken,
    SplitToken,
    OperationToken,
    EndExpressionToken,
}

#[derive(Clone, PartialEq)]
pub enum TokenType {
    // Arithmetic
    OperationToken,

    // Program creation
    LetToken, // variable declaration
    MutableToken, // make variable mutable
    IdentifierToken, // variable name
    EqualToken, // =
    NamespaceAccessToken, // ::
    ScopeBeginToken, // {
    ScopeEndToken, // }
    OpenParenthesisToken, // (
    CloseParenthesisToken, // )
    EndExpressionToken, // ;

    // Types
    BoolToken,
    NumericToken,
    StringToken,
    NameToken,

    // Logic
    IfToken,
    ElseToken,
    ReturnToken,

    // Loops
    WhileToken,
    ForToken,

    // Functions
    FunctionToken,
    PublicToken,
    PrivateToken,
    ProtectedToken,

    // non-registered
    InvalidToken,
    FinishLine,
    SplitToken,
}
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

#[derive(Clone)]
pub struct SplitToken {
    pub token_type: SplitTokenType,
    pub value: String,
}

pub struct Program {
    pub source: String,
    pub tokens: Vec<Token>,
    pub data: HashMap<String, Token>,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token_type {
            TokenType::OperationToken => write!(f, "\x1b[0;31mToken<\x1b[1;31mOperation, \'{}\'\x1b[0;31m>\x1b[0m", self.value),
            TokenType::LetToken => write!(f, "\x1b[0;32mToken\x1b[1;32m<Decl, {}>\x1b[0m", self.value),
            TokenType::NameToken => write!(f, "\x1b[0;33mToken\x1b[1;33m<Name, \"{}\">\x1b[0m", self.value),
            TokenType::NumericToken => write!(f, "\x1b[0;33mToken\x1b[1;33m<Number, {}>\x1b[0m", self.value),
            TokenType::EndExpressionToken => write!(f, "\x1b[0;34mToken\x1b[1;34m<EndExpression, {}>\x1b[0m", self.value),
            _ => write!(f, "Token<{}>", self.value),
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

impl Program {
    pub fn new(source: String) -> Self {
        Program {
            source,
            tokens: vec![],
            data: HashMap::new(),
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
                'a'..='z' | 'A'..='Z' => SplitToken {
                    token_type: SplitTokenType::CharToken,
                    value: token_char.to_string(),
                },
                ' ' | '\t' => SplitToken {
                    token_type: SplitTokenType::SplitToken,
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
                        let next_token = &tokens[next_token_idx];
                        if (next_token.token_type != SplitTokenType::CharToken && next_token.token_type != SplitTokenType::NumToken) || next_token.value == " " {
                            break;
                        }
                        base_str.push_str(&next_token.value);
                        next_token_idx += 1;
                    }
                    
                    let mut current_token_type = TokenType::NameToken;
                    match base_str.to_string().as_str() {
                        "let" => current_token_type = TokenType::LetToken,
                        
                        _ => {}
                    }

                    new_tokens.push(Token {
                        token_type: current_token_type,
                        value: base_str,
                    });
                    
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
                    
                    new_tokens.push(Token {
                        token_type: TokenType::NumericToken,
                        value: base_str,
                    });
                    
                    cur_idx = next_token_idx; 
                },

                // todo: add support for operations that involve mmutliple characters, i. e: !=, ==, etc.
                _ => {    
                    if current_token.token_type == SplitTokenType::EndExpressionToken {
                        new_tokens.push(Token { 
                            token_type: TokenType::EndExpressionToken,
                            value: current_token.value.clone(),
                        });
                    } else {
                        new_tokens.push(Token { 
                            token_type: TokenType::OperationToken,
                            value: current_token.value.clone(),
                        });
                    }

                    cur_idx += 1;
                }
            }
        }

        self.tokens = new_tokens;
    }
    
    pub fn align_to_hashmap(&mut self) {

    }

    pub fn is_token_ending(token: &SplitToken) -> bool {
        token.value == LINE_END_TOKEN || token.value == " "
    }
}


pub fn interpret(source: String) {
    let mut program = Program::new(source);
    program.tokenize();

    println!("{}", program);
}