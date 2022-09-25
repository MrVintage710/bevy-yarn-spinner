use std::fmt::Debug;

pub type YarnResult<T> = Result<T, YarnError>;

#[derive(Clone)]
pub struct YarnError {
    error_name : String,
    error_message : String,
    col : usize,
    line : usize
}

impl Debug for YarnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.gen_error_message().as_str())
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

    pub fn new_invalid_number_error(line : usize, col : usize) -> Self {
        YarnError {
            error_name : "Invalid Number Error".to_string(),
            error_message : "The number at the given line is invalid. Numbers may only contain numerical digits (1-9) and decimals.".to_string(),
            col,
            line,
        }
    }

    pub fn new_invalid_boolean_error(line : usize, col : usize) -> Self {
        YarnError {
            error_name : "Invalid Boolean Error".to_string(),
            error_message : "The boolean at the given line is invalid. Boolean must be either 'true' or 'false'.".to_string(),
            col,
            line,
        }
    }

    pub fn new_variable_not_declared_error(line : usize, col : usize) -> Self {
        YarnError { 
            error_name: "Variable Not Declared Error".to_string(), 
            error_message: "Variable invoked here has not been declared.".to_string(), 
            col, 
            line
        }
    }

    pub fn new_invalid_operation_error(line : usize, col : usize) -> Self {
        YarnError { 
            error_name: "Invalid Opperation Error".to_string(), 
            error_message: "You cannot us this operation on these types.".to_string(), 
            col, 
            line
        }
    }

    pub fn gen_error_message(&self) -> String {
        format!("{} at ({}, {}) : {}", self.error_name, self.line, self.col, self.error_message)
    }

    pub fn error_name(&self) -> &str {
        self.error_name.as_str()
    }

    pub fn error_message(&self) -> &str {
        self.error_message.as_str()
    }
}