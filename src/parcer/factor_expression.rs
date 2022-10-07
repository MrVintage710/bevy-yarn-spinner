use std::result;

use crate::{error::{YarnResult, YarnError}, value::YarnValue, token::{YarnTokenQueue, YarnTokenType}};

use super::{YarnEvaluator, YarnVariableMap, YarnParser, unary_expression::UnaryExpressionNode, primary_expression::YarnValueType, YarnParseResult::{*, self}};

pub enum FactorOperator {
    MUL,
    DIV
}

pub struct FactorExpressionNode {
    lhs : Box<dyn YarnEvaluator>,
    rhs : Box<dyn YarnEvaluator>,
    operator : FactorOperator,
    line : usize,
    col : usize
}

impl FactorExpressionNode {
    pub fn new(lhs : Box<dyn YarnEvaluator>, rhs : Box<dyn YarnEvaluator>, operator : FactorOperator, line : usize, col : usize) -> FactorExpressionNode {
        FactorExpressionNode {
            lhs,
            rhs,
            operator,
            line,
            col,
        }
    }

    pub fn new_boxed(lhs : Box<dyn YarnEvaluator>, rhs : Box<dyn YarnEvaluator>, operator : FactorOperator, line : usize, col : usize) -> Box<FactorExpressionNode> {
        Box::new(FactorExpressionNode::new(lhs, rhs, operator, line, col))
    }
}

impl YarnEvaluator for FactorExpressionNode {
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
                    FactorOperator::MUL => lhs_value.mult(&rhs_value),
                    FactorOperator::DIV => lhs_value.div(&rhs_value),
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

impl YarnParser for FactorExpressionNode {
    fn parse(tokens : &YarnTokenQueue, offset : usize) -> YarnParseResult {
        let lhs = UnaryExpressionNode::parse(tokens, offset);
        if let Parsed(lhs_eval, lhs_endex) = lhs {
            let operator_index = tokens.next_non_space_after(lhs_endex - 1);
            let operator = if tokens.check_index(operator_index, YarnTokenType::FORWARD_SLASH) {
                Some(FactorOperator::DIV)
            } else if tokens.check_index(operator_index, YarnTokenType::MULT) {
                Some(FactorOperator::MUL)
            } else {
                None
            };
            if let Some(operator) = operator {
                let rhs_index = tokens.next_non_space_after(operator_index);
                let rhs = UnaryExpressionNode::parse(tokens, rhs_index);
                if let Parsed(rhs_eval, rhs_endex) = rhs {
                    let factor_expr = FactorExpressionNode::new_boxed(lhs_eval, rhs_eval, operator, tokens.peek_line(offset), tokens.peek_col(offset));
                    return Parsed(factor_expr, rhs_endex)
                }
            }
        }

        UnaryExpressionNode::parse(tokens, offset)
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

        let tokens = tokenize("2 * 2");
        let result = FactorExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::NUMBER(4.0));
                assert_eq!(endex, 6);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("2 / 2");
        let result = FactorExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::NUMBER(1.0));
                assert_eq!(endex, 6);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("true");
        let result = FactorExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::BOOL(true));
                assert_eq!(endex, 2);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("!true");
        let result = FactorExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert_eq!(eval.eval(&mut variables).unwrap().unwrap(), YarnValue::BOOL(false));
                assert_eq!(endex, 3);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("2 / false");
        let result = FactorExpressionNode::parse(&tokens, 1);
        match result {
            Parsed(eval, endex) => {
                assert!(eval.eval(&mut variables).is_err());
                assert_eq!(endex, 6);
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }
    }
}