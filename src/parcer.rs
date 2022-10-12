mod number_literal;
mod string_literal;
mod bool_literal;
mod variable;
mod primary_expression;
mod unary_expression;
mod factor_expression;
mod additive_expression;
mod comparison_expression;
mod equality_expression;
mod command;
mod function;

use std::{collections::HashMap, rc::Rc, fmt::Debug, process::Child};

use crate::{error::{YarnError, YarnResult}, token::{YarnToken, YarnTokenQueue, YarnTokenType::{*, self}}, value::YarnValue};

use self::equality_expression::EqualityExpressionNode;

pub type YarnVariableMap = HashMap<String, YarnValue>;

pub type YarnFunctionMap = HashMap<String, &'static dyn Fn(Vec<YarnValue>) -> YarnResult<Option<YarnValue>>>;

pub enum YarnParseResult {
    Parsed(Box<dyn YarnEvaluator>, usize),
    Error(YarnError),
    Failed
}

pub trait YarnEvaluator {
    fn eval(&self, variables : &mut YarnVariableMap, functions : &YarnFunctionMap) -> YarnResult<Option<YarnValue>>;
}

pub trait YarnExpressionParser {
    fn parse(tokens : &YarnTokenQueue, offset : usize) -> YarnParseResult;
}

pub struct YarnNode {
    first_step : YarnNodeStack,
    headers : HashMap<String, String>,
    title : String
}

pub struct YarnNodeStack {
    lines : Vec<YarnNodeLine>,
    options : Vec<YarnNodeStack>
}

pub enum YarnNodeLine {
    LINE(Option<String>, String),
    COMMAND(Box<dyn YarnEvaluator>)
}

pub struct YarnRuntime {
    nodes : HashMap<String, YarnNode>,
    variables : YarnVariableMap,
    functions : YarnFunctionMap
}

impl YarnRuntime {
    pub fn with_function(mut self, name : &str, function : &'static impl Fn(Vec<YarnValue>) -> YarnResult<Option<YarnValue>>) -> Self {
        self.functions.insert(name.to_string(), function);
        self
    }
}

pub fn parse_expression(tokens : &YarnTokenQueue) -> YarnParseResult {
    EqualityExpressionNode::parse(tokens, 1)
}