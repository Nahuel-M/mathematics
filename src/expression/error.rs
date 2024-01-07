#[derive(Debug, PartialEq)]
pub enum ExpressionError {
    /// The expression cannot be solved because it contains a variable that is not defined
    MissingVariable(String),

}