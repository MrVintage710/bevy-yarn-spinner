#[derive(Debug, PartialEq)]
pub enum YarnValue {
    STRING(String),
    NUMBER(f32),
    BOOL(bool)
}

pub trait YarnValueProvider {
    fn get_value(&self) -> YarnValue;

    fn error(&self) -> Option<String>;
}