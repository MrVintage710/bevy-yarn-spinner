use std::any::Any;

use crate::{value::YarnValue, token::{YarnTokenQueue, YarnTokenType, self}, error::{YarnError, YarnResult}};

use super::{YarnEvaluator, YarnExpressionParser, YarnVariableMap, YarnParseResult::{*, self}, YarnFunctionMap};

pub struct BoolLiteralNode {
    value : bool
}

impl BoolLiteralNode {
    pub fn new(value : bool) -> BoolLiteralNode {
        BoolLiteralNode {
            value
        }
    }

    pub fn new_boxed(value : bool) -> Box<BoolLiteralNode> {
        Box::new(BoolLiteralNode { value })
    }
}

impl YarnEvaluator for BoolLiteralNode {
    fn eval(&self, variables : &mut YarnVariableMap, functions : &YarnFunctionMap) -> Result<Option<YarnValue>, YarnError> {
        Ok(Some(YarnValue::BOOL(self.value)))
    }
}

impl YarnExpressionParser for BoolLiteralNode {
    fn parse(tokens : &YarnTokenQueue, offset : usize) -> YarnParseResult {
        if let Some(token) = tokens.peek(offset) {  
            if token.token_type() == &YarnTokenType::WORD {
                if (token.content() == "true" || token.content() == "false") {
                    if token.content() == "true" {
                        Parsed(BoolLiteralNode::new_boxed(true), offset + 1)
                    } else {
                        Parsed(BoolLiteralNode::new_boxed(false), offset + 1)
                    }
                } else {
                    Error(YarnError::new_invalid_boolean_error(token.line(), token.col()))
                }
            } else {
                Failed
            }
        } else {
            Failed
        }
    }
}

mod tests {
    use crate::{token::tokenize};

    use super::*;

    #[test]
    fn test_parse_bool_literal() {
        let functions = YarnFunctionMap::new();
        let mut variables = YarnVariableMap::new();

        let tokens = tokenize("true");
        let result = BoolLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(endex, 2);
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::BOOL(true));
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("false");
        let result = BoolLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(endex, 2);
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::BOOL(true))
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("Notabool");
        let result = BoolLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(eval, _) => assert!(false),
            Error(error) => {
                assert_eq!(error.error_name(), "Invalid Boolean Error".to_string())
            },
            Failed => assert!(false),
        }

        let tokens = tokenize("");
        let result = BoolLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(eval, _) => assert!(false),
            Error(error) => assert!(false),
            Failed => assert!(true),
        }
    }
}