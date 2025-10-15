use std::collections::HashMap;

#[derive(PartialEq)]
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token_type {
            TokenType::OperationToken => write!(f, "Operation<{}>", self.value),
            TokenType::LetToken => write!(f, "Declaration<{}>", self.value),
            TokenType::NameToken => write!(f, "Name<{}>", self.value),
            TokenType::NumericToken => write!(f, "Number<{}>", self.value),
            _ => write!(f, "Token<{}>", self.value),
        }
    }
}

pub struct Program {
    pub tokens: Vec<Token>,
    pub data: HashMap<String, Token>,
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.tokens.iter() {
            write!(f, "{}\n", token)?;
        }
        Ok(())
    }
}

pub fn is_numeric(token: &str) -> bool {
    token.chars().all(|x| x.is_numeric())
}

pub fn is_bool(token: &str) -> bool {
    token == "true" || token == "false"
}

pub fn tokenize(source: &str) -> Program {
    let program_hashmap = HashMap::new();
    let mut chars = source.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&current_char_token) = chars.peek() {
        if current_char_token.is_ascii_whitespace() { chars.next(); }

        let mut new_token = Token {
            token_type: TokenType::OperationToken,
            value: current_char_token.to_string(),
        };

        if current_char_token.is_ascii_digit() {
            let mut digit_str = String::new();            
            while let Some(&digit_char) = chars.peek() {
                if digit_char.is_ascii_digit() {
                    digit_str.push(digit_char);
                    chars.next();
                } else {
                    break;
                }
            }

            new_token.token_type = TokenType::NumericToken;
            new_token.value = digit_str;
        } else if current_char_token.is_ascii_alphabetic() {
            let mut name_str = String::new();
            while let Some(&name_char) = chars.peek() {
                if name_char.is_ascii_alphabetic() {
                    name_str.push(name_char);
                    chars.next();
                } else {
                    break;
                }
            }

            new_token.token_type = TokenType::NameToken;
            new_token.value = name_str;
        };

        tokens.push(new_token);

        chars.next();
    }

    Program {
        tokens: tokens,
        data: program_hashmap,
    }
}

pub fn interpret(source: &str) {
    let program = tokenize(source);
    
    println!("{}", program);
}