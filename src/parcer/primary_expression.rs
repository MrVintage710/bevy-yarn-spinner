use crate::{error::{YarnResult, YarnError}, token::{YarnTokenQueue, self}};
use super::{YarnParser, variable::VariableNode, string_literal::StringLiteralNode, number_literal::NumberLiteralNode, YarnParseResult::{*, self}, bool_literal::BoolLiteralNode};

pub struct PrimaryExpressionNode;

pub enum YarnValueType {
    VARIABLE,
    STRING,
    NUMBER,
    BOOL
}

impl YarnParser for PrimaryExpressionNode {
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
        
        Failed
    }
    
    
    // type CheckResult = YarnValueType;

    // fn check(tokens : &YarnTokenQueue, offset : usize) -> Option<Self::CheckResult> {
    //     if VariableNode::check(tokens, offset).is_some() {
    //         Some(YarnValueType::VARIABLE)
    //     } else if StringLiteralNode::check(tokens, offset).is_some() {
    //         Some(YarnValueType::STRING)
    //     } else if NumberLiteralNode::check(tokens, offset).is_some() {
    //         Some(YarnValueType::NUMBER)
    //     } else if BoolLiteralNode::check(tokens, offset).is_some() {
    //         Some(YarnValueType::BOOL)
    //     } else {
    //         None
    //     }
    // }

    // fn parse(tokens : &mut YarnTokenQueue, check_result : Self::CheckResult) -> Result<Box<dyn YarnEvaluator>, YarnError> {
    //     match check_result {
    //         YarnValueType::VARIABLE => VariableNode::parse(tokens, ()),
    //         YarnValueType::STRING => StringLiteralNode::parse(tokens, ()),
    //         YarnValueType::NUMBER => NumberLiteralNode::parse(tokens, ()),
    //         YarnValueType::BOOL => BoolLiteralNode::parse(tokens, ()),
    //     }
    // }
}

mod tests {
    use crate::{token::tokenize, value::YarnValue, parcer::YarnVariableMap};

    use super::*;

    #[test]
    fn test_parse_primary_expression() {
        let mut variables = YarnVariableMap::new();
        variables.insert("test".to_string(), YarnValue::BOOL(true));

        let tokens = tokenize("true");
        let result = PrimaryExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(endex, 2);
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::BOOL(true));
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("2.2");
        let result = PrimaryExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(endex, 4);
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::NUMBER(2.2));
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("\"test\"");
        let result = PrimaryExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(endex, 4);
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::STRING("test".to_string()));
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("$test");
        let result = PrimaryExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(endex, 3);
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::BOOL(true));
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }
    }
}