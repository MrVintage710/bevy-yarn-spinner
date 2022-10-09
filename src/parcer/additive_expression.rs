use crate::{error::{YarnResult, YarnError}, value::YarnValue, token::{YarnTokenQueue, YarnTokenType}};

use super::{YarnEvaluator, YarnVariableMap, YarnParser, factor_expression::FactorExpressionNode, YarnParseResult::{*, self}};


pub enum AdditiveOperator {
    PLUS,
    MINUS
}

pub struct AdditiveExpressionNode {
    lhs : Box<dyn YarnEvaluator>,
    rhs : Box<dyn YarnEvaluator>,
    operator : AdditiveOperator,
    line : usize, 
    col : usize
}

impl AdditiveExpressionNode {
    pub fn new(lhs : Box<dyn YarnEvaluator>, rhs : Box<dyn YarnEvaluator>, operator : AdditiveOperator, line : usize, col : usize) -> AdditiveExpressionNode {
        AdditiveExpressionNode {
            lhs,
            rhs,
            operator,
            line,
            col,
        }
    }

    pub fn new_boxed(lhs : Box<dyn YarnEvaluator>, rhs : Box<dyn YarnEvaluator>, operator : AdditiveOperator, line : usize, col : usize) -> Box<AdditiveExpressionNode> {
        Box::new(AdditiveExpressionNode::new(lhs, rhs, operator, line, col))
    }
}

impl YarnEvaluator for AdditiveExpressionNode {
    fn eval(&self, variables : &mut YarnVariableMap) -> YarnResult<Option<YarnValue>> {
        let lhs_value = self.lhs.eval(variables);
        let rhs_value = self.rhs.eval(variables);

        if lhs_value.is_ok() && rhs_value.is_ok() {
            let lhs_value = lhs_value.unwrap();
            let rhs_value = rhs_value.unwrap();
            if lhs_value.is_some() && rhs_value.is_some() {
                let lhs_value = lhs_value.unwrap();
                let rhs_value = rhs_value.unwrap();

                let result = match self.operator {
                    AdditiveOperator::PLUS => lhs_value.add(&rhs_value),
                    AdditiveOperator::MINUS => lhs_value.sub(&rhs_value),
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

impl YarnParser for AdditiveExpressionNode {
    fn parse(tokens : &YarnTokenQueue, offset : usize) -> YarnParseResult {
        let lhs = FactorExpressionNode::parse(tokens, offset);
        if let Parsed(lhs_eval, lhs_endex) = lhs {
            let operator_index = tokens.next_non_space_after(lhs_endex - 1);
            let operator = if tokens.check_index(operator_index, YarnTokenType::SUB) {
                Some(AdditiveOperator::MINUS)
            } else if tokens.check_index(operator_index, YarnTokenType::ADD) {
                Some(AdditiveOperator::PLUS)
            } else {
                None
            };
            if let Some(operator) = operator {
                let rhs_index = tokens.next_non_space_after(operator_index);
                let rhs = FactorExpressionNode::parse(tokens, rhs_index);
                if let Parsed(rhs_eval, rhs_endex) = rhs {
                    let factor_expr = AdditiveExpressionNode::new_boxed(lhs_eval, rhs_eval, operator, tokens.peek_line(offset), tokens.peek_col(offset));
                    return Parsed(factor_expr, rhs_endex)
                }
            }
        }

        FactorExpressionNode::parse(tokens, offset)
    }
}

mod tests {

    use std::env::var;

    use crate::token::tokenize;

    use super::*;

    #[test]
    fn test_parse_variable_literal() {
        let mut variables = YarnVariableMap::new();
        variables.insert("foo".to_string(), YarnValue::NUMBER(2.0));

        let tokens = tokenize("2 + $foo");
        let result = AdditiveExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::NUMBER(4.0));
                assert_eq!(endex, 7);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("2 + $foo * 2");
        let result = AdditiveExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::NUMBER(6.0));
                assert_eq!(endex, 11);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }
    }
}