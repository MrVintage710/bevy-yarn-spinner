use crate::{value::YarnValue, token::{YarnTokenQueue, YarnTokenType::*, self} };

use super::{YarnRuntime, YarnTree};


pub struct StringLiteral {
    value : String
}

impl StringLiteral {
    pub fn new(value : &str) -> Self {
        StringLiteral { value: String::from(value) }
    }

    pub fn new_boxed(value : &str) -> Box<StringLiteral> {
        Box::new(StringLiteral::new(value))
    }
}

impl YarnRuntime for StringLiteral {
    fn eval(&self) -> Option<crate::value::YarnValue> {
        Some(YarnValue::STRING(self.value.clone()))
    }
}

pub fn check_string_literal(tokens : &YarnTokenQueue, offset : usize) -> bool {
    if tokens.peek_type(offset, QUOTATION) {
        let mut index = 1;
        while !tokens.peek_type(offset + index, QUOTATION) 
        && !tokens.peek_type(offset + index, END_LINE) {
            index += 1;
        }

        if tokens.peek_type(offset + index, END_LINE) {
            return false;
        } else {
            return true;
        }
    }

    false
}

pub fn parse_string_literal(tokens : &mut YarnTokenQueue, tree : &mut YarnTree) -> usize {
    tokens.check_and_pop(QUOTATION);

    let mut value = String::new();
    while !tokens.check(QUOTATION) && !tokens.check(END_LINE) {
        if let Some(token) = tokens.pop() {
            value.push_str(token.content())
        }
    }

    tree.add_node(None, StringLiteral::new_boxed(value.as_str()))
}

mod tests {
    use crate::token::tokenize;

    use super::*;

    #[test]
    fn test_check_string_literal() {
        let tokens = tokenize("\"Test\"");
        assert!(check_string_literal(&tokens, 1));

        let tokens = tokenize("\"Test");
        assert!(!check_string_literal(&tokens, 1));
    }

    #[test]
    fn test_parse_string_literal() {
        let mut tokens = tokenize("\"Test\"");
        let mut tree = YarnTree::new();
        tokens.pop();
        let id = parse_string_literal(&mut tokens, &mut tree);
        //println!("{:?}", tree.get_node(id).unwrap().eval())
        assert_eq!(tree.get_node(id).unwrap().eval().unwrap(), YarnValue::STRING("Test".to_string()))
    }
}