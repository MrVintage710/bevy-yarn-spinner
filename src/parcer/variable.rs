use crate::{value::YarnValue, token::{YarnTokenQueue, YarnTokenType}, error::{YarnError, YarnResult}};

use super::{YarnEvaluator, YarnVariableMap, YarnParser, YarnTokenType::*, YarnParseResult::{*, self}};


pub struct VariableNode {
    identifier : String,
    line : usize,
    col : usize
}

impl VariableNode {
    pub fn new(identifier : String, line : usize, col : usize) -> VariableNode {
        VariableNode {
            identifier,
            line,
            col
        }
    }

    pub fn new_boxed(identifier : String, line : usize, col : usize) -> Box<VariableNode> {
        Box::new(VariableNode::new(identifier, line, col))
    }
}

impl YarnEvaluator for VariableNode {
    fn eval(&self, variables : &mut YarnVariableMap) -> Result<Option<YarnValue>, YarnError> {
        if let Some(var) = variables.get(self.identifier.as_str()) {
            Ok(Some(var.clone()))
        } else {
            Err(YarnError::new_variable_not_declared_error(self.line, self.col))
        }
    }
}

impl YarnParser for VariableNode {
    fn parse(tokens : &YarnTokenQueue, offset : usize) -> YarnParseResult {
        if tokens.check_index(offset, YarnTokenType::DOLLAR_SIGN) {
            if let Some(token) = tokens.peek(offset+1) {
                if token.token_type() == &YarnTokenType::WORD {
                    let variable_node = VariableNode::new_boxed(token.content().to_string(), token.line(), token.col());
                    return Parsed(variable_node, offset + 2);
                } else {
                    return Error(YarnError::new_invalid_variable_identifier_error(token.line(), token.col()));
                }
            }
        }
        
        Failed
    }
}

mod tests {

    use std::env::var;

    use crate::token::tokenize;

    use super::*;

    #[test]
    fn test_parse_variable_literal() {
        let mut variables = YarnVariableMap::new();
        variables.insert("test".to_string(), YarnValue::BOOL(true));

        let tokens = tokenize("$test");
        let result = VariableNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::BOOL(true));
                assert_eq!(endex, 3);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("$not_a_var");
        let result = VariableNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                let value = eval.eval(&mut variables);
                assert!(value.is_err());
                assert_eq!(endex, 3);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }
    }
}