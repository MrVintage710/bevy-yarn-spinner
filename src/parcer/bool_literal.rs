use crate::{value::YarnValue, token::{YarnTokenQueue, YarnTokenType}};

use super::{YarnRuntime, YarnTree};

pub struct BoolLiteral {
    value : bool
}

impl BoolLiteral {
    pub fn new(value : bool) -> BoolLiteral {
        BoolLiteral {
            value
        }
    }

    pub fn new_boxed(value : bool) -> Box<BoolLiteral> {
        Box::new(BoolLiteral { value })
    }
}

impl YarnRuntime for BoolLiteral {
    fn eval(&self) -> Option<crate::value::YarnValue> {
        Some(YarnValue::BOOL(self.value))
    }
}

pub fn check_bool_literal(tokens : &YarnTokenQueue, offset : usize) -> bool {
    if let Some(token) = tokens.peek(offset) {
        if token.token_type() == &YarnTokenType::WORD {
            if token.content() == "true" || token.content() == "false" {
                return true;
            }
        }
    }

    false
}

pub fn parse_bool_literal(tokens : &mut YarnTokenQueue, tree : &mut YarnTree) -> usize {
    let value = if let Some(token) = tokens.pop() {
        token.content()
    } else {
        ""
    };

    let b = if value == "true" {
        true
    } else if value == "false" {
        false
    } else {
        false
    };

    tree.add_node(None, BoolLiteral::new_boxed(b))
}

mod tests {
    use crate::token::tokenize;

    use super::*;

    #[test]
    fn test_check_bool_literal() {
        let tokens = tokenize("true");
        assert!(check_bool_literal(&tokens, 1));

        let tokens = tokenize("false");
        assert!(check_bool_literal(&tokens, 1));

        let tokens = tokenize("Not a bool");
        assert!(!check_bool_literal(&tokens, 1))
    }

    #[test]
    fn test_parse_bool_literal() {
        let mut tokens = tokenize("true");
        let mut tree = YarnTree::new();
        tokens.pop();
        let id = parse_bool_literal(&mut tokens, &mut tree);
        assert_eq!(YarnValue::BOOL(true), tree.get_node(id).unwrap().eval().unwrap())
    }
}