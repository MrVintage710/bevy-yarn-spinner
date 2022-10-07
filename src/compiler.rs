use std::{collections::{VecDeque, HashMap}, num};

use crate::{token::{YarnToken, YarnTokenType::{*, self}, YarnTokenQueue}, value::{YarnValue, self}, error::{YarnError, YarnResult}};

//===================================================================================================================================
//                       Helper Functions and Types
//===================================================================================================================================

fn is_str_numeric(string : &str) -> bool {
    string.chars().fold(true, |mut acc , c| acc && c.is_numeric())
}

type YarnVariableMap = HashMap<String, YarnValue>;

//===================================================================================================================================
//                       Compilation Functions - Base Values
//===================================================================================================================================

fn compile_string_value(tokens : &mut YarnTokenQueue) -> Option<YarnResult<YarnValue>> {
    if tokens.check_and_pop(QUOTATION) {
        let mut value = String::new();

        while !tokens.check(QUOTATION) && !tokens.check(END_LINE) {
            value.push_str(tokens.pop().unwrap().content())
        }

        let token = tokens.pop().unwrap();

        if token.token_type() == &END_LINE {
            return Some(Err(YarnError::new_eol_error(token.line(), token.col())))
        }

        if token.token_type() == &QUOTATION {
            return Some(Ok(YarnValue::STRING(value)));
        }
    }

    None
}

fn compile_number_value(tokens : &mut YarnTokenQueue) -> Option<YarnResult<YarnValue>> {
    let mut number = String::new();

    if tokens.check_and_pop(SUB) {
        number.push('-')
    }

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

fn compile_variable_value(tokens : &mut YarnTokenQueue, variables : &mut YarnVariableMap) -> Option<YarnResult<YarnValue>> {
    if tokens.check_and_pop(DOLLAR_SIGN) {
        if tokens.check(WORD) {
            let token = tokens.pop().unwrap();
            if variables.contains_key(token.content()) {
                let value = variables.get(token.content()).unwrap();
                return Some(Ok(value.clone()));
            } else {
                return Some(Err(YarnError::new_variable_not_declared_error(token.line(), token.col())))
            }
        }
    }

    None
}

fn compile_base_value(tokens : &mut YarnTokenQueue, variables : &mut YarnVariableMap) -> Option<YarnResult<YarnValue>> {
    if let Some(value) = compile_variable_value(tokens, variables) {
        Some(value)
    } else if let Some(value) = compile_string_value(tokens) {
        Some(value)
    } else if let Some(value) = compile_number_value(tokens) {
        Some(value)
    } else if let Some(value) = compile_boolean_value(tokens) {
        Some(value)
    } else if let Some(value) = equality_expression(tokens, variables) {
        Some(value)
    } else {
        None
    }
}

//===================================================================================================================================
//                       Compilation Functions - Operations
//===================================================================================================================================

fn detect_duop_group (
    tokens : &mut YarnTokenQueue, 
    variables : &mut YarnVariableMap,
    matcher : impl Fn (
            &mut YarnTokenQueue, 
            &mut YarnVariableMap
        ) -> Option<YarnResult<YarnValue>>
    ) -> Option<(YarnResult<YarnValue>, Option<(YarnTokenType, YarnResult<YarnValue>)>, usize, usize)> {
    let (line, col) = (tokens.peek_line(0), tokens.peek_col(0));
    if let Some(value_1) = matcher(tokens, variables) {
        tokens.remove_leading_spaces();
        if tokens.check(ADD) 
        || tokens.check(SUB) 
        || tokens.check(DIV) 
        || tokens.check(MULT) 
        || tokens.check(EQUAL_TOO)
        || tokens.check(NOT_EQUAL_TOO)
        || tokens.check(LESS_THAN)
        || tokens.check(GREATER_THAN)
        || tokens.check(LESS_THAN_EQ)
        || tokens.check(GREATER_THAN_EQ)
        {
            let operator = tokens.pop().unwrap();
            tokens.remove_leading_spaces();
            if let Some(value_2) = matcher(tokens, variables) {
                return Some((value_1, Some((operator.token_type().clone(), value_2)), line, col));
            }
        } else {
            return Some((value_1, None, line, col));
        }
    }

    None
}

fn equality_expression(tokens : &mut YarnTokenQueue, variables : &mut YarnVariableMap) -> Option<YarnResult<YarnValue>> {
    if let Some((v1, extent, line, col)) = detect_duop_group(tokens, variables, comparison_expression) {
        if !v1.is_err() {
            if let Some((op, v2)) = extent {
                if !v2.is_err() {
                    let v1 = v1.unwrap();
                    let v2 = v2.unwrap();
                    match op {
                        EQUAL_TOO => {
                            if let Some(value) = v1.is_equal(&v2) {
                                return Some(Ok(value));
                            } else {
                                return Some(Err(YarnError::new_invalid_operation_error(line, col)));
                            }
                        },
                        NOT_EQUAL_TOO => {
                            if let Some(value) = v1.is_not_equal(&v2) {
                                return Some(Ok(value));
                            } else {
                                return Some(Err(YarnError::new_invalid_operation_error(line, col)));
                            }
                        },
                        _ => {}
                    }
                } else {
                    return Some(v2);
                }
            } else {
                return Some(v1);
            }
        } else {
            return Some(v1);
        }
    }

    None
}

fn comparison_expression(tokens : &mut YarnTokenQueue, variables : &mut YarnVariableMap) -> Option<YarnResult<YarnValue>> {
    if let Some((v1, extent, line, col)) = detect_duop_group(tokens, variables, additive_expression) {
        if !v1.is_err() {
            if let Some((op, v2)) = extent {
                if !v2.is_err() {
                    let v1 = v1.unwrap();
                    let v2 = v2.unwrap();
                    match op {
                        LESS_THAN => {
                            if let Some(value) = v1.is_less_than(&v2) {
                                return Some(Ok(value));
                            } else {
                                return Some(Err(YarnError::new_invalid_operation_error(line, col)));
                            }
                        },
                        GREATER_THAN => {
                            if let Some(value) = v1.is_greater_than(&v2) {
                                return Some(Ok(value));
                            } else {
                                return Some(Err(YarnError::new_invalid_operation_error(line, col)));
                            }
                        },
                        LESS_THAN_EQ => {
                            if let Some(value) = v1.is_less_than_eq(&v2) {
                                return Some(Ok(value));
                            } else {
                                return Some(Err(YarnError::new_invalid_operation_error(line, col)));
                            }
                        },
                        GREATER_THAN_EQ => {
                            if let Some(value) = v1.is_greater_than_eq(&v2) {
                                return Some(Ok(value));
                            } else {
                                return Some(Err(YarnError::new_invalid_operation_error(line, col)));
                            }
                        },
                        _ => {}
                    }
                } else {
                    return Some(v2);
                }
            } else {
                return Some(v1);
            }
        } else {
            return Some(v1);
        }
    }

    None
}

fn additive_expression(tokens : &mut YarnTokenQueue, variables : &mut YarnVariableMap) -> Option<YarnResult<YarnValue>> {
    if let Some((v1, extent, line, col)) = detect_duop_group(tokens, variables, factor_expression) {
        if !v1.is_err() {
            if let Some((op, v2)) = extent {
                if !v2.is_err() {
                    let v1 = v1.unwrap();
                    let v2 = v2.unwrap();
                    match op {
                        ADD => {
                            if let Some(value) = v1.add(&v2) {
                                return Some(Ok(value));
                            } else {
                                return Some(Err(YarnError::new_invalid_operation_error(line, col)));
                            }
                        },
                        SUB => {
                            if let Some(value) = v1.sub(&v2) {
                                return Some(Ok(value));
                            } else {
                                return Some(Err(YarnError::new_invalid_operation_error(line, col)));
                            }
                        },
                        _ => {}
                    }
                } else {
                    return Some(v2);
                }
            } else {
                return Some(v1);
            }
        } else {
            return Some(v1);
        }
    }

    None
}

fn factor_expression(tokens : &mut YarnTokenQueue, variables : &mut YarnVariableMap) -> Option<YarnResult<YarnValue>> {
    if let Some((v1, extent, line, col)) = detect_duop_group(tokens, variables, compile_base_value) {
        if !v1.is_err() {
            if let Some((op, v2)) = extent {
                if !v2.is_err() {
                    let v1 = v1.unwrap();
                    let v2 = v2.unwrap();
                    match op {
                        MULT => {
                            if let Some(value) = v1.mult(&v2) {
                                return Some(Ok(value));
                            } else {
                                return Some(Err(YarnError::new_invalid_operation_error(line, col)));
                            }
                        },
                        DIV => {
                            if let Some(value) = v1.div(&v2) {
                                return Some(Ok(value));
                            } else {
                                return Some(Err(YarnError::new_invalid_operation_error(line, col)));
                            }
                        },
                        _ => {return Some(Ok(v1));}
                    }
                } else {
                    return Some(v2);
                }
            } else {
                return Some(v1);
            }
        } else {
            return Some(v1);
        }
    }

    None
}

//===================================================================================================================================
//                       Tests
//===================================================================================================================================

#[cfg(test)]
mod tests {
    use crate::token::tokenize;

    use super::*;

    #[test]
    fn compile_equality_test() {
        let mut tokens = tokenize("1+1*2");
        println!("{:?}", tokens);
        let mut variables = YarnVariableMap::new();
        tokens.pop();
        let value = equality_expression(&mut tokens, &mut variables).unwrap().unwrap();
        println!("{:?}", value);
        assert_eq!(value, YarnValue::NUMBER(3.0))
    }

    #[test]
    fn compile_bool_test() {
        let mut tokens = tokenize("true");
        tokens.pop();
        let value = compile_boolean_value(&mut tokens).unwrap().unwrap();
        assert_eq!(value, YarnValue::BOOL(true))
    }

    #[test]
    fn compile_number_test() {
        let mut tokens = tokenize("-2.2");
        tokens.pop();
        let value = compile_number_value(&mut tokens).unwrap().unwrap();
        assert_eq!(value, YarnValue::NUMBER(-2.2));

        let mut tokens = tokenize("2.invalid");
        tokens.pop();
        let value = compile_number_value(&mut tokens).unwrap();
        assert!(value.is_err());
    }

    #[test]
    fn compile_string_test() {
        let mut tokens = tokenize("\"This is a test String.\"");
        tokens.pop();
        let value = compile_string_value(&mut tokens).unwrap().unwrap();
        assert_eq!(value, YarnValue::STRING("This is a test String.".to_string()));

        let mut tokens = tokenize("\"Forgot to end this string");
        tokens.pop();
        let value = compile_string_value(&mut tokens).unwrap();
        assert!(value.is_err());
    }

    #[test]
    fn compile_variable_test() {
        let mut tokens = tokenize("$test");
        let mut variables = YarnVariableMap::new();
        variables.insert("test".to_string(), YarnValue::NUMBER(2.2));
        tokens.pop();
        let value = compile_variable_value(&mut tokens, &mut variables).unwrap().unwrap();
        assert_eq!(value, YarnValue::NUMBER(2.2));
    }
}
