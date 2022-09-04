use std::{collections::VecDeque, fs::read_to_string};

use parser::parse_yarn_string;

use crate::parser::parse_line;

mod parser;

fn main() {
    let test = r"
        Test
    ";

    println!("{}", test.contains("\n"));

    let source = read_to_string("assets/simple_test.yarn").unwrap();

    let result = parse_yarn_string(source.as_str());

    for token in result.iter() {
        println!("{:?}", token)
    }
}
