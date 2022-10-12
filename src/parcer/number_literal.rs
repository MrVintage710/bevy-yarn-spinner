use core::num;

use crate::{value::YarnValue, token::{YarnTokenQueue, YarnTokenType}, error::{YarnError}};

use super::{YarnEvaluator, YarnVariableMap, YarnExpressionParser, YarnParseResult::{self, *}, YarnFunctionMap};

#[derive(Debug)]
pub struct NumberLiteralNode {
    value : f32
}

impl  NumberLiteralNode {
    pub fn new(value : f32) -> NumberLiteralNode {
        NumberLiteralNode {
            value
        }
    }

    pub fn new_boxed(value : f32) -> Box<NumberLiteralNode> {
        Box::new(
            NumberLiteralNode {
                value
            }
        )
    }
}

impl YarnEvaluator for NumberLiteralNode {
    fn eval(&self, variables : &mut YarnVariableMap, functions : &YarnFunctionMap) -> Result<Option<YarnValue>, YarnError> {
        Ok(Some(YarnValue::NUMBER(self.value)))
    }
}

impl YarnExpressionParser for NumberLiteralNode {
    fn parse(tokens : &YarnTokenQueue, offset : usize) -> YarnParseResult {
        if let Some(integral) = tokens.peek_only_if_type(offset, YarnTokenType::WORD) {
            if integral.is_numeric() {
                let mut value = String::from(integral.content());
                let mut len = 1;

                if tokens.check_index(offset + 1, YarnTokenType::PERIOD) {
                    if let Some(fractional) = tokens.peek_only_if_type(offset+2, YarnTokenType::WORD) {
                        if fractional.is_numeric() {
                            value.push_str(".");
                            value.push_str(fractional.content());
                            len += 2;
                        } else {
                            return Error(YarnError::new_invalid_number_error(fractional.line(), fractional.col()));
                        }
                    }
                }

                let eval = NumberLiteralNode::new_boxed(value.parse::<f32>().unwrap());
                return Parsed(eval, offset+len);
            }
        }

        Failed
    }
}

#[cfg(test)]
mod tests {

    use crate::{token::tokenize, value::YarnValue};

    use super::*;

    #[test]
    fn test_parse_number_literal() {
        let functions = YarnFunctionMap::new();
        let mut variables = YarnVariableMap::new();

        let tokens = tokenize("2");
        let result = NumberLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::NUMBER(2.0));
                assert_eq!(endex, 2)
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("2.2");
        let result = NumberLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::NUMBER(2.2));
                assert_eq!(endex, 4)
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("2.test");
        let result = NumberLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(_, _) => assert!(false),
            Error(_) => assert!(true),
            Failed => assert!(false),
        }

        let tokens = tokenize("test.2");
        let result = NumberLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(_, _) => assert!(false),
            Error(_) => assert!(false),
            Failed => assert!(true),
        }

        let tokens = tokenize("");
        let result = NumberLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => assert!(false),
            Error(_) => assert!(false),
            Failed => assert!(true),
        }
    }
}