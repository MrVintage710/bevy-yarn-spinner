use std::collections::VecDeque;

use bevy::{utils::HashMap, ecs::world::WorldCell, app::AppLabel};

use crate::{token::{YarnToken, YarnTokenType::*}, value::YarnValue, error::YarnError};

//===================================================================================================================================
//                       Helper Functions
//===================================================================================================================================

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

fn is_str_numeric(string : &str) -> bool {
    string.chars().fold(true, |mut acc , c| acc && c.is_numeric())
}

//===================================================================================================================================
//                       Compilation Functions
//===================================================================================================================================

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

        if first_token_is_any!(tokens, EOF) || tokens.is_empty() {
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

fn compile_number_value(tokens : &mut VecDeque<YarnToken>) -> Option<Result<YarnValue, YarnError>>{
    let mut number = String::new();

    if check_tokens!(tokens, WORD) {
        if let Some(first_token) = tokens.pop_front() {
            if is_str_numeric(first_token.contents()) {
                number.push_str(first_token.contents());
                if check_pop_tokens!(tokens, PERIOD) {
                    number.push('.');
                    if check_tokens!(tokens, WORD) {
                        if let Some(second_token) = tokens.pop_front() {
                            if is_str_numeric(second_token.contents()) {
                                number.push_str(second_token.contents())
                            } else {
                                return Some(Err(YarnError::new_invalid_number_error(second_token.line(), second_token.col_start())));
                            }
                        }
                    }
                }

                return Some(Ok(YarnValue::NUMBER(number.parse().unwrap())));
            }
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
        assert!(check_tokens!(tokens, START_LINE, LESS_THAN, LESS_THAN, END_LINE, EOF));
    }

    #[test]
    fn compile_line_test() {
        
    }

    #[test]
    fn compile_string_value_test() {
        let mut tokens = tokenize_yarn_string("\"This is a test line\"");
        tokens.pop_front();
        let value = compile_string_value(&mut tokens).unwrap().unwrap();
        assert_eq!(value, YarnValue::STRING("This is a test line".to_string()));

        let mut tokens = tokenize_yarn_string("\"This is a test line");
        tokens.pop_front();
        let value = compile_string_value(&mut tokens).unwrap();
        assert!(value.is_err());
        assert_eq!(value.unwrap_err().error_name(), "EOL Error");

        let mut tokens = tokenize_yarn_string("\"This is a test line\nand this is another\"");
        tokens.pop_front(); 
        let value = compile_string_value(&mut tokens).unwrap();
        assert!(value.is_err());
        assert_eq!(value.unwrap_err().error_name(), "EOL Error");
    }

    #[test]
    fn compile_number_test() {
        let mut tokens = tokenize_yarn_string("3.14");
        tokens.pop_front();
        let value = compile_number_value(&mut tokens).unwrap();
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), YarnValue::NUMBER(3.14));

        let mut tokens = tokenize_yarn_string("2.test");
        tokens.pop_front();
        let value = compile_number_value(&mut tokens).unwrap();
        assert!(value.is_err());
        assert_eq!(value.unwrap_err().error_name(), "Invalid Number Error");

        let mut tokens = tokenize_yarn_string("Not a Numeber");
        tokens.pop_front();
        let value = compile_number_value(&mut tokens);
        assert!(value.is_none());
    }

    #[test]
    fn is_numeric_function() {
        assert!(is_str_numeric("1234"))
    }
}
