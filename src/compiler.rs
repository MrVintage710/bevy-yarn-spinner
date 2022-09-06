use std::collections::VecDeque;

use bevy::utils::HashMap;

use crate::{token::{YarnToken, YarnTokenType::*}, value::YarnValue, error::YarnError};

macro_rules! first_token_is_any {
    ($tokens:ident, $($token:ident),*) => {
        {
            let mut _result = false;
            let t = $tokens.front();
            if let Some(t) = t {
                $(
                    if !_result && &$token == t.token_type() {
                        _result = true;
                    }
                )*
            }

            _result
        }
    };
}

macro_rules! check_tokens {
    ($tokens:ident, $($token:ident),*) => {
        {
            let mut _index = 0;
            let mut _result = true;
            $(
                if _result {
                    if let Some(t) =  $tokens.get(_index) {
                        _result = t.token_type() == &$token
                    } else {
                        _result = false;
                    }
                    _index += 1;
                }
            )*

            _result
        }
    }
}

macro_rules! check_pop_tokens {
    ($tokens:ident, $($token:ident),*) => {
        {
            let mut _result = true;
            let mut _index = 0;
            $(
                if _result {
                    if let Some(t) =  $tokens.get(_index) {
                        if t.token_type() == &$token {
                            &$tokens.pop_front();
                        } else {
                            _result = false;
                        }
                    } else {
                        _result = false;
                    }
                }
                _index += 1;
            )*

            _result
        }
    };
}

pub struct YarnOperation {
    
}

pub struct YarnNode {
    title : String,
    //lines : Vec<>
}

pub fn create_yarn_operation<'a>(tokens : &VecDeque<YarnToken<'a>>) {
    let variables : HashMap<String, YarnValue> = HashMap::new();

    for token in tokens.iter() {
        
    }
}

fn compile_line(mut tokens : VecDeque<YarnToken>) {
    if let Some(token) = tokens.front() {
        
    }
}

fn compile_string_value(tokens : &mut VecDeque<YarnToken>) -> Option<Result<YarnValue, YarnError>> {
    if check_pop_tokens!(tokens, QUOTATION) {
        let mut value = String::new();
        let mut error = false;
        while !first_token_is_any!(tokens, QUOTATION, EOF, END_LINE) {
            let token = tokens.pop_front().unwrap();
            value.push_str(token.contents());
        }

        if first_token_is_any!(tokens, EOF) {
            let token = tokens.front().unwrap();
            return Some(Err(YarnError::new_eof_error(token.line(), token.col_start())));
        }

        if first_token_is_any!(tokens, END_LINE) {
            let token = tokens.front().unwrap();
            return Some(Err(YarnError::new_eol_error(token.line(), token.col_start())));
        }

        if check_pop_tokens!(tokens, QUOTATION) {
            return Some(Ok(YarnValue::STRING(value)));
        }
    }

    None
}

//===================================================================================================================================
//                       Tests
//===================================================================================================================================

#[cfg(test)]
mod tests {
    use crate::token::tokenize_yarn_string;

    use super::*;

    #[test]
    fn has_any_macro() {
        let mut tokens = tokenize_yarn_string("test");
        println!("{:?}, {}", tokens, check_tokens!(tokens, START_LINE, LESS_THAN, LESS_THAN));
        assert!(first_token_is_any!(tokens, START_LINE, LESS_THAN))
    }

    #[test]
    fn check_tokens_macro() {
        let mut tokens = tokenize_yarn_string("<<");
        println!("{:?}, {}", tokens, check_tokens!(tokens, START_LINE, LESS_THAN, LESS_THAN));
        assert!(check_tokens!(tokens, LESS_THAN, LESS_THAN))
    }

    #[test]
    fn compile_line_test() {
        
    }

    #[test]
    fn compile_string_value_test() {
        let mut tokens = tokenize_yarn_string("\"This is a test line\"");
        tokens.pop_front();
        tokens.pop_back();
        tokens.pop_back();
        let value = compile_string_value(&mut tokens).unwrap().unwrap();
        assert_eq!(value, YarnValue::STRING("This is a test line".to_string()));

        let mut tokens = tokenize_yarn_string("\"This is a test line");
        tokens.pop_front();
        tokens.pop_back();
        let value = compile_string_value(&mut tokens).unwrap();
        assert!(value.is_err())
    }
}
