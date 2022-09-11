use std::{collections::VecDeque, num};

use bevy::{utils::HashMap, ecs::world::WorldCell, app::AppLabel};

use crate::{token::{YarnToken, YarnTokenType::*, YarnTokenQueue}, value::YarnValue, error::{YarnError, YarnResult}};

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

//===================================================================================================================================
//                       Compilation Functions - Base Values
//===================================================================================================================================

fn compile_string_value(tokens : &mut YarnTokenQueue) -> Option<YarnResult<YarnValue>> {
    // if check_pop_tokens!(tokens, QUOTATION) {
    //     let mut value = String::new();
    //     let mut error = false;
    //     while !first_token_is_any!(tokens, QUOTATION, EOF, END_LINE) {
    //         let token = tokens.pop_front().unwrap();
    //         value.push_str(&token.contents().copy_data());
    //     }

    //     if first_token_is_any!(tokens, EOF) || tokens.is_empty() {
    //         let token = tokens.front().unwrap();
    //         return Some(Err(YarnError::new_eof_error(token.line(), token.col_start())));
    //     }

    //     if first_token_is_any!(tokens, END_LINE) {
    //         let token = tokens.front().unwrap();
    //         return Some(Err(YarnError::new_eol_error(token.line(), token.col_start())));
    //     }

    //     if check_pop_tokens!(tokens, QUOTATION) {
    //         return Some(Ok(YarnValue::STRING(value)));
    //     }
    // }

    None
}

fn compile_number_value(tokens : &mut YarnTokenQueue) -> Option<YarnResult<YarnValue>> {
    let mut number = String::new();

    if tokens.check(WORD) {
        if let Some(first_token) = tokens.pop() {
            if is_str_numeric(first_token.content()) {
                number.push_str(first_token.content());
                if tokens.check_and_pop(PERIOD) {
                    number.push('.');
                    if tokens.check(WORD) {
                        if let Some(second_token) = tokens.pop() {
                            if is_str_numeric(second_token.content()) {
                                number.push_str(second_token.content())
                            } else {
                                return Some(Err(YarnError::new_invalid_number_error(first_token.line(), first_token.col())))
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

fn compile_boolean_value(tokens : &mut YarnTokenQueue) -> Option<YarnResult<YarnValue>> {
    if tokens.check(WORD) {
        let token = tokens.pop().unwrap();
        if token.content() == "true" {
            return Some(Ok(YarnValue::BOOL(true)));
        } else if token.content() == "false" {
            return Some(Ok(YarnValue::BOOL(false)));
        } else {
            return Some(Err(YarnError::new_invalid_boolean_error(token.line(), token.col())));
        };
    }
    None
}

//===================================================================================================================================
//                       Compilation Functions - Operations
//===================================================================================================================================

// fn compile_duop_value(tokens : &mut VecDeque<YarnToken>) -> Option<YarnResult<YarnValue>> {

//     let first_value = compile_number_value(tokens);
//     remove_leading_spaces(tokens);
//     let operaptor = if first_token_is_any!(tokens, MULT, DIV, ADD, SUB) {

//     };

//     None
// }

//===================================================================================================================================
//                       Tests
//===================================================================================================================================

#[cfg(test)]
mod tests {
    use crate::token::tokenize;

    use super::*;

    #[test]
    fn compile_bool_test() {
        let mut tokens = tokenize("true");
        tokens.pop();
        let value = compile_boolean_value(&mut tokens).unwrap().unwrap();
        assert_eq!(value, YarnValue::BOOL(true))
    }

    #[test]
    fn compile_number_test() {
        let mut tokens = tokenize("2.2");
        tokens.pop();
        let value = compile_number_value(&mut tokens).unwrap().unwrap();
        assert_eq!(value, YarnValue::NUMBER(2.2))
    }
}
