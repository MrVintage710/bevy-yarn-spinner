use std::process::Child;

use crate::{error::{YarnResult, YarnError}, value::{YarnValue, self}, token::{YarnTokenQueue, self, YarnTokenType}};

use super::{YarnEvaluator, YarnVariableMap, YarnParser, YarnParseResult::{*, self}, primary_expression::PrimaryExpressionNode};

enum UnaryOperator {
    NOT,
    NEGATIVE
}

pub struct UnaryExpressionNode {
    operator : UnaryOperator,
    child : Box<dyn YarnEvaluator>,
    line : usize,
    col : usize
}

impl UnaryExpressionNode {
    fn new(child : Box<dyn YarnEvaluator>, operator : UnaryOperator, line : usize, col : usize) -> UnaryExpressionNode {
        UnaryExpressionNode {
            operator,
            child,
            line,
            col,
        }
    }

    fn new_boxed(child : Box<dyn YarnEvaluator>, operator : UnaryOperator, line : usize, col : usize) -> Box<UnaryExpressionNode> {
        Box::new(UnaryExpressionNode::new(child, operator, line, col))
    }
}

impl YarnEvaluator for UnaryExpressionNode {
    fn eval(&self, variables : &mut YarnVariableMap) -> YarnResult<Option<YarnValue>> {
        let value = self.child.eval(variables);

        match value {
            Ok(value) => {
                if let Some(value) = value {
                    if let UnaryOperator::NOT = self.operator {
                        match value {
                            YarnValue::STRING(_) => Err(YarnError::new_invalid_operation_error(self.line, self.col)),
                            YarnValue::NUMBER(_) => Err(YarnError::new_invalid_operation_error(self.line, self.col)),
                            YarnValue::BOOL(boolean) => Ok(Some(YarnValue::BOOL(!boolean))),
                        }
                    } else {
                        match value {
                            YarnValue::STRING(_) => Err(YarnError::new_invalid_operation_error(self.line, self.col)),
                            YarnValue::NUMBER(number) => Ok(Some(YarnValue::NUMBER(-number))),
                            YarnValue::BOOL(_) => Err(YarnError::new_invalid_operation_error(self.line, self.col)),
                        }
                    }
                } else {
                    Ok(None)
                }
            },
            Err(error) => Err(error)
        }
    }
}

impl YarnParser for UnaryExpressionNode {
    fn parse(tokens : &YarnTokenQueue, offset : usize) -> YarnParseResult {
        let operator = if tokens.check_index(offset, YarnTokenType::SUB) {
            Some(UnaryOperator::NEGATIVE)
        } else if tokens.check_index(offset, YarnTokenType::BANG) {
            Some(UnaryOperator::NOT)
        } else {
            None
        };

        if let Some(operator) = operator {
            let line = tokens.peek_line(offset);
            let col = tokens.peek_col(offset);
            let result = PrimaryExpressionNode::parse(tokens, offset + 1);
            if let Parsed(eval, endex) = result {
                Parsed(UnaryExpressionNode::new_boxed(eval, operator, line, col), endex)
            } else {
                result
            }
        } else {
            PrimaryExpressionNode::parse(tokens, offset)
        } 
    }
}

mod tests {
    use crate::token::tokenize;

    use super::*;

    #[test]
    fn test_parse_primary_expression() {
        let mut variables = YarnVariableMap::new();

        let tokens = tokenize("!true");
        let result = UnaryExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::BOOL(false));
                assert_eq!(endex, 3);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("-2.2");
        let result = UnaryExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::NUMBER(-2.2));
                assert_eq!(endex, 5);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("-\"test\"");
        let result = UnaryExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                let value = eval.eval(&mut variables);
                assert!(value.is_err());
                assert_eq!(endex, 5);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }
    }
}