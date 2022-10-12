use crate::{error::YarnError, token::YarnTokenType};

use super::{YarnEvaluator, YarnExpressionParser, comparison_expression::ComparisonExpressionNode, YarnParseResult::{*, self}, YarnFunctionMap};

pub enum EqualityOperator {
    EQUAL_TOO,
    NOT_EQUAL_TOO
}

pub struct EqualityExpressionNode {
    lhs : Box<dyn YarnEvaluator>,
    rhs : Box<dyn YarnEvaluator>,
    operator : EqualityOperator,
    line : usize,
    col : usize
}

impl EqualityExpressionNode {
    pub fn new(lhs : Box<dyn YarnEvaluator>, rhs : Box<dyn YarnEvaluator>, operator : EqualityOperator, line : usize, col : usize) -> EqualityExpressionNode {
        EqualityExpressionNode {
            lhs,
            rhs,
            operator,
            line,
            col,
        }
    }

    pub fn new_boxed(lhs : Box<dyn YarnEvaluator>, rhs : Box<dyn YarnEvaluator>, operator : EqualityOperator, line : usize, col : usize) -> Box<EqualityExpressionNode> {
        Box::new(EqualityExpressionNode::new(lhs, rhs, operator, line, col))
    }
}

impl YarnEvaluator for EqualityExpressionNode {
    fn eval(&self, variables : &mut super::YarnVariableMap, functions : &YarnFunctionMap) -> crate::error::YarnResult<Option<crate::value::YarnValue>> {
        let lhs_value = self.lhs.eval(variables, functions);
        let rhs_value = self.rhs.eval(variables, functions);

        if lhs_value.is_ok() && rhs_value.is_ok() {
            let lhs_value = lhs_value.unwrap();
            let rhs_value = rhs_value.unwrap();
            if lhs_value.is_some() && rhs_value.is_some() {
                let lhs_value = lhs_value.unwrap();
                let rhs_value = rhs_value.unwrap();

                let result = match self.operator {
                    EqualityOperator::EQUAL_TOO => lhs_value.is_equal(&rhs_value),
                    EqualityOperator::NOT_EQUAL_TOO => lhs_value.is_not_equal(&rhs_value),
                };

                if let Some(result) = result {
                    Ok(Some(result))
                } else {
                    Err(YarnError::new_invalid_operation_error(self.line, self.col))
                }
            } else {
                Err(YarnError::new_invalid_operation_error(self.line, self.col))
            }
        } else {
            if lhs_value.is_err() {
                lhs_value
            } else {
                rhs_value
            }
        }
    }
}

impl YarnExpressionParser for EqualityExpressionNode {
    fn parse(tokens : &crate::token::YarnTokenQueue, offset : usize) -> YarnParseResult {
        let lhs = ComparisonExpressionNode::parse(tokens, offset);
        if let Parsed(lhs_eval, lhs_endex) = lhs {
            let operator_index = tokens.next_non_space_after(lhs_endex - 1);
            let operator = if tokens.check_index(operator_index, YarnTokenType::EQUAL_TOO) {
                Some(EqualityOperator::EQUAL_TOO)
            } else if tokens.check_index(operator_index, YarnTokenType::NOT_EQUAL_TOO) {
                Some(EqualityOperator::NOT_EQUAL_TOO)
            } else {
                None
            };
            if let Some(operator) = operator {
                let rhs_index = tokens.next_non_space_after(operator_index);
                let rhs = ComparisonExpressionNode::parse(tokens, rhs_index);
                if let Parsed(rhs_eval, rhs_endex) = rhs {
                    let factor_expr = EqualityExpressionNode::new_boxed(lhs_eval, rhs_eval, operator, tokens.peek_line(offset), tokens.peek_col(offset));
                    return Parsed(factor_expr, rhs_endex)
                }
            }
        }

        ComparisonExpressionNode::parse(tokens, offset)
    }
}

mod tests {

    use std::env::var;

    use crate::{token::tokenize, parcer::YarnVariableMap, value::YarnValue};

    use super::*;

    #[test]
    fn test_parse_variable_literal() {
        let functions = YarnFunctionMap::new();
        let mut variables = YarnVariableMap::new();
        variables.insert("test".to_string(), YarnValue::BOOL(true));

        let tokens = tokenize("2 == 2");
        let result = EqualityExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::BOOL(true));
                assert_eq!(endex, 6);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("10 * 10 == 5 * (2 + 18)");
        let result = EqualityExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::BOOL(true));
                //assert_eq!(endex, 6);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }
    }
}