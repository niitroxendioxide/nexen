use crate::language::tokens::*;

fn process_name_token(token_str: &str) -> Token {
    match token_str {
        "let" | "local" => Token::LetToken(token_str.to_string()),
        "if" => Token::IfToken(token_str.to_string()),
        "else" => Token::ElseToken(token_str.to_string()),
        "while" => Token::WhileToken(token_str.to_string()),
        "for" => Token::ForToken(token_str.to_string()),
        "function" => Token::FunctionToken(token_str.to_string()),
        "public" => Token::PublicToken(token_str.to_string()),
        "private" => Token::PrivateToken(token_str.to_string()),
        "protected" => Token::ProtectedToken(token_str.to_string()),
        "be" => Token::EqualToken(token_str.to_string()),
        "do" => Token::ScopeBeginToken("{".to_string()),
        "end" => Token::ScopeEndToken("}".to_string()),
        "return" => Token::ReturnToken(token_str.to_string()),
        "loop" => Token::LoopToken(token_str.to_string()),
        "continue" => Token::ContinueToken(token_str.to_string()),
        "break" => Token::BreakToken(token_str.to_string()),
        "true" | "false" => Token::BoolToken(token_str.to_string()),
        _ => Token::IdentifierToken(token_str.to_string()),
    }
}

pub fn tokenize(new_tokens: &mut Vec<Token>, tokens: &Vec<SplitToken>) {
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
                
                let mut is_fn_def = false;
                while next_token_idx < tokens.len() {
                    let next_token: &SplitToken = &tokens[next_token_idx];
                    
                    if next_token.value == "(" {
                        is_fn_def = true;
                    }

                    if !is_fn_def && (next_token.token_type == SplitTokenType::SplitToken 
                            || next_token.token_type == SplitTokenType::EndExpressionToken
                            || next_token.token_type == SplitTokenType::OperationToken) {
                        break;
                    }

                    base_str.push_str(&next_token.value);
                    next_token_idx += 1;

                    if is_fn_def && next_token.token_type == SplitTokenType::OperationToken
                        && next_token.value == ")" { 
                        break;
                    }
                }
                
                let current_token = process_name_token(&base_str);

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

                let current_token = Token::StringToken(base_str);
                new_tokens.push(current_token);
                
                cur_idx = next_token_idx; 
            },
            SplitTokenType::NumToken => {
                let mut base_str = String::new();
                let mut next_token_idx = cur_idx;
                
                while next_token_idx < tokens.len() {
                    let next_token = &tokens[next_token_idx];
                    if next_token.value == "_" {
                        next_token_idx += 1;
                        continue;
                    }
                    
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
}