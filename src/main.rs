use std::{collections::VecDeque, fs::read_to_string};

use token::tokenize_yarn_string;

mod token;
mod compiler;
mod value;
mod error;

fn main() {
    let test = r"
        Test
    ";

    println!("{}", test.contains("\n"));

    let source = read_to_string("assets/simple_test.yarn").unwrap();

    let result = tokenize_yarn_string(source.as_str());

    for token in result.iter() {
        println!("{:?}", token)
    }
}
