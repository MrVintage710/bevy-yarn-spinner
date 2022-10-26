use std::i16::MIN;

use rand::Rng;

use crate::{error::{YarnResult, YarnError}, value::YarnValue::{*, self}, token::{YarnTokenQueue, YarnTokenType}};
use super::{YarnEvaluator, YarnVariableMap, YarnFunctionMap, YarnExpressionParser, YarnParseResult::{*, self}, parse_expression, equality_expression::EqualityExpressionNode};

pub struct FunctionNode {
    arguments : Vec<Box<dyn YarnEvaluator>>,
    function_name : String,
    line : usize,
    col : usize
}

impl FunctionNode {
    pub fn new(arguments : Vec<Box<dyn YarnEvaluator>>, function_name : String, line : usize, col : usize) -> FunctionNode {
        FunctionNode {
            arguments,
            function_name,
            line,
            col,
        }
    }

    pub fn new_boxed(arguments : Vec<Box<dyn YarnEvaluator>>, function_name : String, line : usize, col : usize) -> Box<FunctionNode> {
        Box::new(FunctionNode::new(arguments, function_name, line, col))
    }
}

impl YarnEvaluator for FunctionNode {
    fn eval(&self, variables : &mut YarnVariableMap, functions : &YarnFunctionMap) -> YarnResult<Option<YarnValue>> {
        if functions.contains_key(&self.function_name) {
            let mut values = Vec::new();
            for eval in self.arguments.iter() {
                match eval.eval(variables, functions) {
                    Ok(v) => match v {
                        Some(v) => values.push(v),
                        None => return Err(YarnError::new_null_function_arg_error(self.line, self.col)),
                    },
                    Err(error) => return Err(error),
                }
            }
            functions.get(&self.function_name).unwrap()(values, self.line, self.col)
        } else {
            Err(YarnError::new_undefined_function_error(self.line, self.col))
        }
    }
}

impl YarnExpressionParser for FunctionNode {
    fn parse(tokens : &YarnTokenQueue, offset : usize) -> YarnParseResult {
        if let Some(function_id) = tokens.peek(offset) {
            if tokens.check_index(offset + 1, YarnTokenType::LEFT_PAREN) {
                let mut evals = Vec::new();
                let args_start = offset + 2;
                let mut args_offset = 0;
                let mut ran_into_comma = false;

                loop {
                    let current_index = args_offset + args_start;
                    if tokens.check_index(current_index, YarnTokenType::RIGHT_PAREN) {
                        break;
                    }

                    match EqualityExpressionNode::parse(tokens, current_index) {
                        Parsed(eval, endex) => {
                            evals.push(eval);
                            args_offset += endex - current_index;
                        },
                        Error(err) => {
                            return Error(err);
                        },
                        Failed => {
                            if tokens.check_index(current_index, YarnTokenType::COMMA) {
                                if ran_into_comma {
                                    return Error(YarnError::new_unexpected_token_error(
                                        tokens.peek_line(current_index),
                                        tokens.peek_col(current_index)
                                    ));
                                } else {
                                    ran_into_comma = true;
                                    args_offset += 1;
                                }
                            } else if tokens.check_index(current_index, YarnTokenType::SPACE) {
                                args_offset += 1
                            } else {
                                return Error(YarnError::new_unexpected_token_error(
                                    tokens.peek_line(current_index),
                                    tokens.peek_col(current_index)
                                ));
                            };
                        },
                    }
                }

                Parsed(FunctionNode::new_boxed(
                    evals,
                    function_id.content().to_string(),
                    tokens.peek_line(offset),
                    tokens.peek_col(offset)
                ), offset + args_start + args_offset)
            } else {
                Failed
            }
        } else {
            Failed
        }
    }
}

//==================================================================================================================
//                   Default Yarn Functions
//==================================================================================================================

macro_rules! check_arg {
    ($args:ident, $index:expr, $type:ident, $line:expr, $col:expr) => {
        {
            let mut result = &YarnValue::BOOL(true);

            if let Some(value) = $args.get($index) {
                if let $type(_) = value {
                    result = value;
                } else {
                    return Err(YarnError::new_type_mismatch_error($line, $col, stringify!($type), value.get_type_as_string()));
                }
            } else {
                return Err(YarnError::new_null_function_arg_error($line, $col));
            }

            result
        }
    };
}

pub fn dice(arguments : Vec<YarnValue>, line : usize, col : usize) -> YarnResult<Option<YarnValue>> {
    let mut rng = rand::thread_rng();
    let sides = check_arg!(arguments, 0, NUMBER, line, col);
    if let Some(sides) = arguments.get(0) {
        if let YarnValue::NUMBER(sides) = sides {
            Ok(Some(YarnValue::NUMBER(rng.gen_range(0.0..=*sides).round())))
        } else {
            Err(YarnError::new_type_mismatch_error(line, col, "NUMBER", sides.get_type_as_string()))
        }
    } else {
        Err(YarnError::new_null_function_arg_error(line, col))
    }
}

pub fn random(arguments : Vec<YarnValue>, line : usize, col : usize) -> YarnResult<Option<YarnValue>> {
    let mut rng = rand::thread_rng();
    Ok(Some(YarnValue::NUMBER(rng.gen())))
}

pub fn random_range(arguments : Vec<YarnValue>, line : usize, col : usize) -> YarnResult<Option<YarnValue>> {
    let min = check_arg!(arguments, 0, NUMBER, line, col).as_f64().unwrap();
    let max = check_arg!(arguments, 1, NUMBER, line, col).as_f64().unwrap();

    println!("{} .. {} | {:?}", min, max, min..=max);

    let mut rng = rand::thread_rng();
    Ok(Some(YarnValue::NUMBER(rng.gen_range(min..=max))))
}

pub fn round(arguments : Vec<YarnValue>, line : usize, col : usize) -> YarnResult<Option<YarnValue>> {
    let value = check_arg!(arguments, 0, NUMBER, line, col).as_f64().unwrap();
    Ok(Some(NUMBER(value.round())))
}

pub fn round_places(arguments : Vec<YarnValue>, line : usize, col : usize) -> YarnResult<Option<YarnValue>> {
    let value = check_arg!(arguments, 0, NUMBER, line, col).as_f64().unwrap();
    let places = check_arg!(arguments, 1, NUMBER, line, col).as_f64().unwrap();

    let signifigance = 10.0_f64.powi(places as i32);
    let value = ((value * signifigance).round()) / signifigance;

    Ok(Some(NUMBER(value)))
}

pub fn floor(arguments : Vec<YarnValue>, line : usize, col : usize) -> YarnResult<Option<YarnValue>> {
    let value = check_arg!(arguments, 0, NUMBER, line, col).as_f64().unwrap();
    Ok(Some(NUMBER(value.floor())))
}

pub fn ceil(arguments : Vec<YarnValue>, line : usize, col : usize) -> YarnResult<Option<YarnValue>> {
    let value = check_arg!(arguments, 0, NUMBER, line, col).as_f64().unwrap();
    Ok(Some(NUMBER(value.ceil())))
}

pub fn inc(arguments : Vec<YarnValue>, line : usize, col : usize) -> YarnResult<Option<YarnValue>> {
    let mut value = check_arg!(arguments, 0, NUMBER, line, col).as_f64().unwrap();

    if value.fract() != 0.0 {
        value = value.ceil()
    } else {
        value += 1.0;
    }

    Ok(Some(NUMBER(value)))
}

pub fn dec(arguments : Vec<YarnValue>, line : usize, col : usize) -> YarnResult<Option<YarnValue>> {
    let mut value = check_arg!(arguments, 0, NUMBER, line, col).as_f64().unwrap();

    if value.fract() != 0.0 {
        value = value.floor()
    } else {
        value -= 1.0;
    }

    Ok(Some(NUMBER(value)))
}

pub fn decimal(arguments : Vec<YarnValue>, line : usize, col : usize) -> YarnResult<Option<YarnValue>> {
    let value = check_arg!(arguments, 0, NUMBER, line, col).as_f64().unwrap();
    Ok(Some(NUMBER(value - value.floor())))
}

#[cfg(test)]
mod tests {

    use std::env::var;

    use crate::{token::tokenize, parcer::{YarnFunctionMap, default_function_map}};

    use super::*;

    #[test]
    fn test_parse_variable_literal() {
        let mut functions = default_function_map();
        let mut variables = YarnVariableMap::new();
        variables.insert("foo".to_string(), YarnValue::NUMBER(2.0));

        let tokens = tokenize("dice(6)");
        let eval = FunctionNode::parse(&tokens, 1);
        match eval {
            Parsed(eval, endex) => {
                let value = eval.eval(&mut variables, &functions).unwrap().unwrap();
                assert!(value >= YarnValue::NUMBER(0.0) && value <= YarnValue::NUMBER(6.0));
            },
            Error(err) => {
                println!("{}", err.gen_error_message());
                assert!(false)
            },
            Failed => assert!(false),
        }

        let tokens = tokenize("random_range(0, 2)");
        let eval = FunctionNode::parse(&tokens, 1);
        match eval {
            Parsed(eval, endex) => {
                let value = eval.eval(&mut variables, &functions).unwrap().unwrap();
                assert!(value >= YarnValue::NUMBER(0.0) && value <= YarnValue::NUMBER(2.0));
            },
            Error(_) => assert!(false),
            Failed => assert!(false),
        }

        let tokens = tokenize("round(2.2)");
        let eval = FunctionNode::parse(&tokens, 1);
        match eval {
            Parsed(eval, endex) => {
                let value = eval.eval(&mut variables, &functions).unwrap().unwrap();
                assert!(value == YarnValue::NUMBER(2.0));
            },
            Error(e) => {
                println!("{}", e.gen_error_message());
                assert!(false);
            }
            Failed => assert!(false),
        }

        let tokens = tokenize("round_places(2.24, 1)");
        let eval = FunctionNode::parse(&tokens, 1);
        match eval {
            Parsed(eval, endex) => {
                let value = eval.eval(&mut variables, &functions).unwrap().unwrap();
                assert!(value == YarnValue::NUMBER(2.2));
            },
            Error(e) => {
                println!("{}", e.gen_error_message());
                assert!(false);
            }
            Failed => assert!(false),
        }

        let tokens = tokenize("floor(2.24)");
        let eval = FunctionNode::parse(&tokens, 1);
        match eval {
            Parsed(eval, endex) => {
                let value = eval.eval(&mut variables, &functions).unwrap().unwrap();
                assert!(value == YarnValue::NUMBER(2.0));
            },
            Error(e) => {
                println!("{}", e.gen_error_message());
                assert!(false);
            }
            Failed => assert!(false),
        }

        let tokens = tokenize("ceil(2.24)");
        let eval = FunctionNode::parse(&tokens, 1);
        match eval {
            Parsed(eval, endex) => {
                let value = eval.eval(&mut variables, &functions).unwrap().unwrap();
                assert!(value == YarnValue::NUMBER(3.0));
            },
            Error(e) => {
                println!("{}", e.gen_error_message());
                assert!(false);
            }
            Failed => assert!(false),
        }

        let tokens = tokenize("inc(2)");
        let eval = FunctionNode::parse(&tokens, 1);
        match eval {
            Parsed(eval, endex) => {
                let value = eval.eval(&mut variables, &functions).unwrap().unwrap();
                assert!(value == YarnValue::NUMBER(3.0));
            },
            Error(e) => {
                println!("{}", e.gen_error_message());
                assert!(false);
            }
            Failed => assert!(false),
        }

        let tokens = tokenize("dec(2)");
        let eval = FunctionNode::parse(&tokens, 1);
        match eval {
            Parsed(eval, endex) => {
                let value = eval.eval(&mut variables, &functions).unwrap().unwrap();
                assert!(value == YarnValue::NUMBER(1.0));
            },
            Error(e) => {
                println!("{}", e.gen_error_message());
                assert!(false);
            }
            Failed => assert!(false),
        }

        let tokens = tokenize("decimal(5.7)");
        let eval = FunctionNode::parse(&tokens, 1);
        match eval {
            Parsed(eval, endex) => {
                let value = eval.eval(&mut variables, &functions).unwrap().unwrap();
                println!("{:?}", value);
                assert!(value == YarnValue::NUMBER(0.7));
            },
            Error(e) => {
                println!("{}", e.gen_error_message());
                assert!(false);
            }
            Failed => assert!(false),
        }
    }

    pub fn test(args : Vec<YarnValue>, line : usize, col : usize) -> YarnResult<Option<YarnValue>> {
        Ok(Some(YarnValue::BOOL(true)))
    }
}