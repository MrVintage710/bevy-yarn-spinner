mod number_literal;
mod string_literal;
mod bool_literal;

use std::{collections::HashMap, rc::Rc, fmt::Debug};

use crate::{error::YarnError, token::{YarnToken, YarnTokenQueue, YarnTokenType::{*, self}}, value::YarnValue};

type YarnVariableMap = HashMap<String, YarnValue>;
pub trait YarnRuntime {
    fn eval(&self) -> Option<YarnValue>;
}

pub struct YarnNode {
    id : usize,
    error : Option<YarnError>,
    parent : Option<usize>,
    childern : Vec<usize>,
    runtime : Box<dyn YarnRuntime>
}

impl Debug for YarnNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YarnNode").field("id", &self.id).finish()
    }
}

impl YarnNode {
    pub fn eval(&self) -> Option<YarnValue> {
        self.runtime.eval()
    }

    pub fn get_child_id(&self, index : usize) -> Option<&usize> {
        self.childern.get(index)
    }
}

pub struct YarnTree {
    nodes : Vec<YarnNode>
}

impl <'a> YarnTree {
    pub fn new() -> Self {
        YarnTree { nodes: Vec::new() }
    }

    pub fn add_node<T>(&mut self, parent : Option<usize>, runtime : Box<T>) -> usize
    where T : YarnRuntime  + 'static{
        let index = self.nodes.len();

        if parent.is_some() {
            self.add_child(parent.unwrap(), index)
        }

        let node = YarnNode {
            id: index,
            parent,
            childern: vec![],
            runtime,
            error : None
        };

        self.nodes.push(node);

        index
    }

    pub fn get_child(&self, parent_id : usize, child_index : usize) -> Option<&YarnNode> {
        if let Some(parent_node) = self.get_node(parent_id) {
            if let Some(child_id) = parent_node.get_child_id(child_index) {
                return self.get_node(child_index);
            }
        }

        None
    }

    pub fn add_child(&mut self, parent_id : usize, child_id : usize) {
        self.get_node_mut(parent_id).unwrap().childern.push(child_id);
        self.add_parent(child_id, parent_id);
    }

    pub fn add_parent(&mut self, child_id : usize, parent_id : usize) {
        self.get_node_mut(child_id).unwrap().parent = Some(parent_id);
        self.add_child(parent_id, child_id);
    }

    pub fn get_node_mut(&mut self, node_id : usize) -> Option<&mut YarnNode> {
        self.nodes.get_mut(node_id)
    }

    pub fn get_node(&self, node_id : usize) -> Option<&YarnNode> {
        self.nodes.get(node_id)
    }

    pub fn search_graph(&self, path : &str, start_index : usize) -> Option<&YarnNode>{
        if path.is_empty() {
            return self.get_node(start_index);
        }
        
        let index = path.find("-");
        let child_id = if let Some(index) = index {
            let next_id = &path[0..index];
            next_id.parse().expect("Invalid Tree Reference")
        } else {
            path.parse().expect("Invalid Tree Reference")
        };

        let current_node = self.get_node(start_index).unwrap().get_child_id(child_id);
        if let None = current_node {
            return None;
        }
        if let Some(index) = index {
            self.search_graph(&path[index+1..], *current_node.unwrap())
        } else {
            self.search_graph("", *current_node.unwrap())
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::{*, number_literal::NumberLiteral};

    #[test]
    fn test_tree() {
        let mut tree = YarnTree::new();
        let first_node = tree.add_node(None, NumberLiteral::new_boxed(1.0));
        let second_node = tree.add_node(Some(first_node), NumberLiteral::new_boxed(2.0));
        let third_node = tree.add_node(Some(first_node), NumberLiteral::new_boxed(3.0));
        let fourth_node = tree.add_node(Some(second_node), NumberLiteral::new_boxed(4.0));
        let fith_node = tree.add_node(Some(third_node), NumberLiteral::new_boxed(5.0));

        println!("{:?}", tree.search_graph("1-0", first_node).is_some())
    }
}




//Number
// pub struct NumberElement {
//     value : f32
// }

// impl YarnRuntime for NumberElement {
    // fn check(tokens : &YarnTokenQueue, offset : usize) -> bool {
    //     let mut number_start = if tokens.peek_type(offset, YarnTokenType::SUB) {
    //         1
    //     } else {
    //         0
    //     };
        
    //     if tokens.peek_type(offset + number_start, YarnTokenType::WORD) {
    //         let first_number_token = tokens.peek(offset + number_start).unwrap();
    //         if first_number_token.is_numeric() {
                
                
    //             return ;
    //         }
    //     }

    //     None
    // }

//     fn eval(&self) -> Option<YarnValue> {
//         Some(YarnValue::NUMBER(self.value))
//     }
// }
// 69
// fn check(tokens : &YarnTokenQueue, offset : usize) -> bool {
//     let mut number_start = if tokens.peek_type(offset, YarnTokenType::SUB) {
//         1
//     } else {
//         0
//     };
    
//     if tokens.peek_type(offset + number_start, YarnTokenType::WORD) {
//         let first_number_token = tokens.peek(offset + number_start).unwrap();
//         if first_number_token.is_numeric() {
            
            
//             return ;
//         }
//     }

//     None
// }