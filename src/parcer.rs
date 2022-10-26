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

use std::{collections::{HashMap, VecDeque}, rc::Rc, fmt::Debug, process::Child};
use crate::{error::{YarnError, YarnResult}, token::{YarnToken, YarnTokenQueue, YarnTokenType::{*, self}}, value::YarnValue};
use self::equality_expression::EqualityExpressionNode;

pub type YarnVariableMap = HashMap<String, YarnValue>;

pub type YarnFunction = &'static dyn Fn(Vec<YarnValue>, usize, usize) -> YarnResult<Option<YarnValue>>;

pub type YarnFunctionMap = HashMap<String, YarnFunction>;

pub fn default_function_map() -> YarnFunctionMap {
    let mut functions = YarnFunctionMap::new();
    functions.insert("dice".to_string(), &function::dice);
    functions.insert("random".to_string(), &function::random);
    functions.insert("random_range".to_string(), &function::random_range);
    functions.insert("round".to_string(), &function::round);
    functions.insert("round_places".to_string(), &function::round_places);
    functions.insert("floor".to_string(), &function::floor);
    functions.insert("ceil".to_string(), &function::ceil);
    functions.insert("inc".to_string(), &function::inc);
    functions.insert("dec".to_string(), &function::dec);
    functions.insert("decimal".to_string(), &function::decimal);

    functions
}

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
    lines : VecDeque<YarnNodeLine>,
    options : Vec<YarnNodeStack>
}

pub enum YarnNodeLine {
    LINE(Option<String>, String, Vec<String>), //Speaker Name, Line Text, Tags
    COMMAND(Box<dyn YarnEvaluator>), // The Command evaluator
    EMPTY
}

pub struct YarnRuntime {
    nodes : HashMap<String, YarnNode>,
    variables : YarnVariableMap,
    functions : YarnFunctionMap,
}

impl YarnRuntime {
    pub fn new(source : &str) -> YarnRuntime {
        YarnRuntime { 
            nodes: HashMap::new(), 
            variables: YarnVariableMap::new(), 
            functions: default_function_map() 
        }
    }
}

impl From<&str> for YarnRuntime {
    fn from(source : &str) -> Self {
        YarnRuntime::new(source)
    }
}

impl YarnRuntime {
    pub fn with_function(mut self, name : &str, function : YarnFunction) -> Self {
        self.functions.insert(name.to_string(), function);
        self
    }
}

pub fn parse_expression(tokens : &YarnTokenQueue) -> YarnParseResult {
    EqualityExpressionNode::parse(tokens, 1)
}