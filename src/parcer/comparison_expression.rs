use crate::{error::YarnError, token::{YarnTokenQueue, YarnTokenType}};

use super::{YarnEvaluator, YarnExpressionParser, YarnParseResult::{*, self}, factor_expression::FactorExpressionNode, additive_expression::AdditiveExpressionNode, YarnFunctionMap};

pub enum ComparisonOperator {
    LESS_THAN,
    GREATER_THAN,
    GREATER_THAN_EQ,
    LESS_THAN_EQ
}

pub struct ComparisonExpressionNode {
    lhs : Box<dyn YarnEvaluator>,
    rhs : Box<dyn YarnEvaluator>,
    operator : ComparisonOperator,
    line : usize,
    col : usize
}

impl ComparisonExpressionNode {
    pub fn new(lhs : Box<dyn YarnEvaluator>, rhs : Box<dyn YarnEvaluator>, operator : ComparisonOperator, line : usize, col : usize) -> ComparisonExpressionNode {
        ComparisonExpressionNode {
            lhs,
            rhs,
            operator,
            line,
            col,
        }
    }

    pub fn new_boxed(lhs : Box<dyn YarnEvaluator>, rhs : Box<dyn YarnEvaluator>, operator : ComparisonOperator, line : usize, col : usize) -> Box<ComparisonExpressionNode> {
        Box::new(ComparisonExpressionNode::new(lhs, rhs, operator, line, col))
    }
}

impl YarnEvaluator for ComparisonExpressionNode {
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
                    ComparisonOperator::LESS_THAN => lhs_value.is_less_than(&rhs_value),
                    ComparisonOperator::GREATER_THAN => lhs_value.is_greater_than(&rhs_value),
                    ComparisonOperator::GREATER_THAN_EQ => lhs_value.is_greater_than_eq(&rhs_value),
                    ComparisonOperator::LESS_THAN_EQ => lhs_value.is_less_than_eq(&rhs_value),
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

impl YarnExpressionParser for ComparisonExpressionNode {
    fn parse(tokens : &YarnTokenQueue, offset : usize) -> YarnParseResult {
        let lhs = AdditiveExpressionNode::parse(tokens, offset);
        if let Parsed(lhs_eval, lhs_endex) = lhs {
            let operator_index = tokens.next_non_space_after(lhs_endex - 1);
            let operator = if tokens.check_index(operator_index, YarnTokenType::GREATER_THAN) {
                Some(ComparisonOperator::GREATER_THAN)
            } else if tokens.check_index(operator_index, YarnTokenType::LESS_THAN) {
                Some(ComparisonOperator::GREATER_THAN)
            } else if tokens.check_index(operator_index, YarnTokenType::GREATER_THAN_EQ) {
                Some(ComparisonOperator::GREATER_THAN_EQ)
            } else if tokens.check_index(operator_index, YarnTokenType::LESS_THAN_EQ) {
                Some(ComparisonOperator::LESS_THAN_EQ)
            } else {
                None
            };
            if let Some(operator) = operator {
                let rhs_index = tokens.next_non_space_after(operator_index);
                let rhs = AdditiveExpressionNode::parse(tokens, rhs_index);
                if let Parsed(rhs_eval, rhs_endex) = rhs {
                    let factor_expr = ComparisonExpressionNode::new_boxed(lhs_eval, rhs_eval, operator, tokens.peek_line(offset), tokens.peek_col(offset));
                    return Parsed(factor_expr, rhs_endex)
                }
            }
        }

        AdditiveExpressionNode::parse(tokens, offset)
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

        let tokens = tokenize("3 > 2");
        let result = ComparisonExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::BOOL(true));
                assert_eq!(endex, 6);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("3 >= 2 + 1");
        let result = ComparisonExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::BOOL(true));
                assert_eq!(endex, 10);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("3 * 3 <= 2");
        let result = ComparisonExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables, &functions).unwrap().unwrap(), YarnValue::BOOL(false));
                assert_eq!(endex, 10);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }
    }
}