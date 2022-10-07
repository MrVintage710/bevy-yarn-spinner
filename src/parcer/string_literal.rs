use crate::{value::{YarnValue, self}, token::{YarnTokenQueue, YarnTokenType::{*, self}, self}, error::{YarnError, YarnResult} };

use super::{YarnEvaluator, YarnVariableMap, YarnParser, YarnParseResult::{*, self}};


pub struct StringLiteralNode {
    value : String
}

impl StringLiteralNode {
    pub fn new(value : &str) -> Self {
        StringLiteralNode { value: String::from(value) }
    }

    pub fn new_boxed(value : &str) -> Box<StringLiteralNode> {
        Box::new(StringLiteralNode::new(value))
    }
}

impl YarnEvaluator for StringLiteralNode {
    fn eval(&self, variables : &mut YarnVariableMap) -> Result<Option<YarnValue>, YarnError> {
        Ok(Some(YarnValue::STRING(self.value.clone())))
    }
}

impl YarnParser for StringLiteralNode {
    fn parse(tokens : &YarnTokenQueue, offset : usize) -> YarnParseResult {
        if tokens.check_index(offset, YarnTokenType::QUOTATION) {
            let mut cursor = 1;
            let mut content = String::new();
            let mut escape_character = false;
            
            while let Some(token) = tokens.peek(offset + cursor) {
                if token.token_type() == &YarnTokenType::QUOTATION && !escape_character {
                    break;
                }
                
                if token.token_type() == &YarnTokenType::END_LINE {
                    return Error(YarnError::new_eol_error(token.line(), token.col()));
                }

                escape_character = if token.token_type() == &YarnTokenType::BACKWARD_SLASH && !escape_character { 
                    true
                } else {
                    false
                };

                if !escape_character {
                    content.push_str(token.content())
                }

                cursor += 1
            }

            return Parsed(StringLiteralNode::new_boxed(content.as_str()), offset + cursor + 1);
        }
        
        Failed
    }
}

mod tests {
    use crate::token::tokenize;

    use super::*;

    #[test]
    fn test_parse_string_literal() {
        let mut variables = YarnVariableMap::new();

        let tokens = tokenize("\"test\"");
        let result = StringLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::STRING("test".to_string()));
                assert_eq!(endex, 4)
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("\"test\\\"\"");
        let result = StringLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::STRING("test\"".to_string()));
                assert_eq!(endex, 6)
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("\"test with multiple words\"");
        let result = StringLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::STRING("test with multiple words".to_string()));
                assert_eq!(endex, 10)
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("\"test with multiple words");
        let result = StringLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(_, _) => assert!(false),
            Error(_) => assert!(true),
            Failed => assert!(false),
        }

        let tokens = tokenize("");
        let result = StringLiteralNode::parse(&tokens, 1);
        match result {
            Parsed(_, _) => assert!(false),
            Error(_) => assert!(false),
            Failed => assert!(true),
        }
    }
}