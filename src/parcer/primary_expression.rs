use crate::{error::{YarnResult, YarnError}, token::{YarnTokenQueue, self, YarnTokenType}};
use super::{YarnExpressionParser, variable::VariableNode, string_literal::StringLiteralNode, number_literal::NumberLiteralNode, YarnParseResult::{*, self}, bool_literal::BoolLiteralNode, equality_expression::EqualityExpressionNode, function::FunctionNode};

pub struct PrimaryExpressionNode;

pub enum YarnValueType {
    VARIABLE,
    STRING,
    NUMBER,
    BOOL
}

impl YarnExpressionParser for PrimaryExpressionNode {
    fn parse(tokens : &YarnTokenQueue, offset : usize) -> YarnParseResult {
        let variable_eval = VariableNode::parse(tokens, offset);
        match variable_eval {
            Failed => {},
            _ => { return variable_eval }
        }

        let string_eval = StringLiteralNode::parse(tokens, offset);
        match string_eval {
            Failed => {},
            _ => { return string_eval }
        }

        let number_eval = NumberLiteralNode::parse(tokens, offset);
        match number_eval {
            Failed => {},
            _ => { return number_eval }
        }

        let bool_eval = BoolLiteralNode::parse(tokens, offset);
        match bool_eval {
            Failed => {},
            _ => { return bool_eval }
        }

        let function_eval = FunctionNode::parse(tokens, offset);
        match function_eval {
            Failed => {},
            _ => { return function_eval }
        }

        if tokens.check_index(offset, YarnTokenType::LEFT_PAREN) {
            let result = EqualityExpressionNode::parse(tokens, offset + 1);
            match result {
                Parsed(eval, endex) => {
                    if tokens.check_index(endex, YarnTokenType::RIGHT_PAREN) {
                        return Parsed(eval, endex + 1);
                    } else {
                        return Error(YarnError::new_unexpected_token_error(tokens.peek_line(endex), tokens.peek_col(endex)))
                    }
                },
                Error(error) => return Error(error),
                Failed => {},
            }
        }
        
        Failed
    }
}

mod tests {
    use crate::{token::tokenize, value::YarnValue, parcer::{YarnVariableMap, YarnFunctionMap}};

    use super::*;

    #[test]
    fn test_parse_primary_expression() {
        let functions = YarnFunctionMap::new();
        let mut variables = YarnVariableMap::new();
        variables.insert("test".to_string(), YarnValue::BOOL(true));

        let tokens = tokenize("true");
        let result = PrimaryExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(endex, 2);
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::BOOL(true));
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("2.2");
        let result = PrimaryExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(endex, 4);
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::NUMBER(2.2));
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("\"test\"");
        let result = PrimaryExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(endex, 4);
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::STRING("test".to_string()));
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("$test");
        let result = PrimaryExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(endex, 3);
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::BOOL(true));
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("($test == true)");
        let result = PrimaryExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(endex, 9);
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::BOOL(true));
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }
    }
}