mod number_literal;
mod string_literal;
mod bool_literal;
mod variable;
mod primary_expression;
mod unary_expression;
mod factor_expression;
mod additive_expression;
mod comparison_expression;

use std::{collections::HashMap, rc::Rc, fmt::Debug, process::Child};

use crate::{error::{YarnError, YarnResult}, token::{YarnToken, YarnTokenQueue, YarnTokenType::{*, self}}, value::YarnValue};

type YarnVariableMap = HashMap<String, YarnValue>;

pub enum YarnParseResult {
    Parsed(Box<dyn YarnEvaluator>, usize),
    Error(YarnError),
    Failed
}

pub trait YarnEvaluator {
    fn eval(&self, variables : &mut YarnVariableMap) -> YarnResult<Option<YarnValue>>;
}

pub trait YarnParser {
    fn parse(tokens : &YarnTokenQueue, offset : usize) -> YarnParseResult;
}