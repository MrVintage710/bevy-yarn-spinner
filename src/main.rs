use std::collections::VecDeque;

use crate::parser::parse_line;

mod parser;

fn main() {
    let mut tokens = VecDeque::default();

    parse_line(&mut tokens, "<<if $tam >>", 0, 0, 0);
}
