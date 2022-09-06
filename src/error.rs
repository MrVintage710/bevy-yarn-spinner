use std::fmt::Debug;

#[derive(Clone)]
pub struct YarnError {
    error_name : String,
    error_message : String,
    col : usize,
    line : usize
}

impl Debug for YarnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_error_message().as_str())
    }
}

impl YarnError {

    pub fn new_eof_error(line : usize, col : usize) -> Self {
        YarnError {
            error_name : "EOF Error".to_string(),
            error_message : "There was an end of file before a string was closed.".to_string(),
            col,
            line,
        }
    }

    pub fn new_eol_error(line : usize, col : usize) -> Self {
        YarnError {
            error_name : "EOL Error".to_string(),
            error_message : "There was an end of the line before a string was closed.".to_string(),
            col,
            line,
        }
    }

    pub fn get_error_message(&self) -> String {
        format!("{} at ({}, {}) : {}", self.error_name, self.line, self.col, self.error_message)
    }
}