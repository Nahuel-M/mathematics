#[derive(Debug, PartialEq)]
pub enum ExpressionError {
    MissingVariable(String),
}
