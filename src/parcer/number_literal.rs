use core::num;

use crate::{value::YarnValue, token::{YarnTokenQueue, YarnTokenType::*, self}, error::YarnError};

use super::{YarnRuntime, YarnVariableMap, YarnNode, YarnTree};

#[derive(Debug)]
pub struct NumberLiteral {
    value : f32
}

impl  NumberLiteral {
    pub fn new(value : f32) -> NumberLiteral {
        NumberLiteral {
            value
        }
    }

    pub fn new_boxed(value : f32) -> Box<NumberLiteral> {
        Box::new(
            NumberLiteral {
                value
            }
        )
    }
}

impl YarnRuntime for NumberLiteral {
    fn eval(&self) -> Option<YarnValue> {
        Some(YarnValue::NUMBER(self.value))
    }
}

pub fn check_number_literal(tokens : &YarnTokenQueue, offset : usize) -> bool {
    let mut number_start = if tokens.peek_type(offset, SUB) {
        1
    } else {
        0
    };
    
    if tokens.peek_type(offset + number_start, WORD) {
        let first_number_token = tokens.peek(offset + number_start).unwrap();
        if first_number_token.is_numeric() {
            return true;
        }
    }

    false
}

pub fn parse_number(tokens : &mut YarnTokenQueue, tree : &mut YarnTree, ) -> usize {
    let negative = tokens.check_and_pop(SUB);

    let mut value = if negative { "-".to_string()} else {String::new()};
    if let Some(token) = tokens.pop_if_type(WORD) {
        if token.is_numeric() {
            value.push_str(token.content())
        }
    } 
    
    if let Some(_) = tokens.pop_if_type(PERIOD) {
        value.push('.')
    }

    if let Some(token) = tokens.pop_if_type(WORD) {
        if token.is_numeric() {
            value.push_str(token.content())
        }
    } 

    let number  = value.parse::<f32>();
    let runtime = if number.is_err() {
        NumberLiteral {
            value : 0.0
        }
    } else {
        NumberLiteral {
            value : number.unwrap()
        }
    };

    tree.add_node(None, Box::new(runtime))
}

#[cfg(test)]
mod tests {
    use crate::token::tokenize;

    use super::*;

    #[test]
    fn test_check_number_literal() {
        let tokens = tokenize("2");
        assert!(check_number_literal(&tokens, 1));

        let tokens = tokenize("2.2");
        assert!(check_number_literal(&tokens, 1));

        let tokens = tokenize("-2");
        assert!(check_number_literal(&tokens, 1));
    }

    #[test]
    fn test_parse_number_literal() {
        let mut tokens = tokenize("2");
        let mut tree = YarnTree::new();
        tokens.pop();
        let token_index = parse_number(&mut tokens, &mut tree);
        assert_eq!(tree.get_node(token_index).unwrap().eval().unwrap(), YarnValue::NUMBER(2.0));

        let mut tokens = tokenize("2.2");
        let mut tree = YarnTree::new();
        tokens.pop();
        let token_index = parse_number(&mut tokens, &mut tree);
        assert_eq!(tree.get_node(token_index).unwrap().eval().unwrap(), YarnValue::NUMBER(2.2));

        let mut tokens = tokenize("-2.2");
        let mut tree = YarnTree::new();
        tokens.pop();
        let token_index = parse_number(&mut tokens, &mut tree);
        assert_eq!(tree.get_node(token_index).unwrap().eval().unwrap(), YarnValue::NUMBER(-2.2));
    }
}