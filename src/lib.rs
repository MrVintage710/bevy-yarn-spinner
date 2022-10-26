use std::{collections::VecDeque, fs::read_to_string, io::Write};

use error::YarnResult;
use parcer::{parse_expression, YarnParseResult, YarnFunctionMap, default_function_map};
use token::tokenize;
use value::YarnValue;

use crate::{parcer::{YarnVariableMap, }};

use yarn_spinner_macros::yarn_function;

mod token;
mod value;
mod error;
mod parcer;

// fn main() {
//     let mut source = String::new();
//     let mut variables = YarnVariableMap::new();
//     let mut functions = default_function_map();
//     //let mut result = YarnParseResult::Failed;

//     loop {
//         let input = promt("Yarn > ");
//         let mut args = input.trim().split(" ").collect::<VecDeque<&str>>();
//         let command = args.pop_front().unwrap();

//         if command == "source" {
//             source = String::new();
//             let tokens = source_mode(&mut source);
//             println!("{:?}", tokens);
//             let result = parse_expression(&tokens);
//             match result {
//                 YarnParseResult::Parsed(eval, _) => {
//                     match eval.eval(&mut variables, &functions) {
//                         Ok(value) => println!("{:?}", value),
//                         Err(error) => println!("Runtime Err | {}", error.gen_error_message()),
//                     }
//                 },
//                 YarnParseResult::Error(error) => println!("Compile Err | {}", error.gen_error_message()),
//                 YarnParseResult::Failed => println!("Did not Parse correctly. Try again!"),
//             }
//         } else if command == "set" {
//             if args.get(0).is_some() && args.get(1).is_some() {
//                 let variable_name = args.get(0).unwrap().trim().to_string();
//                 let value = args.get(1).unwrap().trim().into();
//                 println!("Variable '{}' set too {:?}", variable_name, value);
//                 variables.insert(variable_name, value);
//             } else {
//                 println!("Must have a name for the variable and a value to set it to.")
//             }
//         } else if command == "quit" || command == "exit" {
//             break;
//         } else {
//             println!("That is not a valid command. Type help for information.")
//         }
//     }
// }

// fn source_mode<'a>(source : &'a mut String) -> token::YarnTokenQueue<'a> {
//     let mut current_line : u32 = 0;

//     while true {
//         let line = promt(format!("{:3}| ", current_line).as_str());
//         if line.trim() == "done" {
//             break;
//         } else {
//             source.push_str(line.as_str())
//         }
//         current_line += 1;
//     }

//     let tokens = tokenize(source.as_str());
//     tokens
// }

// fn promt(prefix : &str) -> String {
//     std::io::stdout().write(prefix.as_bytes()).unwrap();
//     std::io::stdout().flush().unwrap();

//     let mut line = String::new();
//     std::io::stdin().read_line(&mut line).unwrap();
//     line
// }

#[cfg(test)]
mod tests {
    use yarn_spinner_macros::yarn_function;

    use crate::{token::tokenize, parcer::YarnRuntime};

    use super::*;

    #[test]
    fn main_test() {
        test(vec![YarnValue::BOOL(true), YarnValue::BOOL(false)], 0, 0).unwrap();

        let tokens = tokenize("1+1");
        let eval = parse_expression(&tokens);

        let runtime = YarnRuntime::new("source").with_function("test", &test);
    }

    #[yarn_function]
    fn test(test : i32, test_2 : u32) {
        println!("{:?}", test);
        println!("{:?}", test_2);
        Ok(None)
    }
}