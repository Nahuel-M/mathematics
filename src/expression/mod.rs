use std::collections::HashMap;
use std::fmt::Display;
use crate::expression::add::Add;
use crate::expression::invert::Invert;
use crate::expression::multiply::Multiply;
use crate::expression::negate::Negate;
use crate::expression::number::Number;

use self::{constant::Constant, error::ExpressionError};
pub mod constant;
pub mod display;
pub mod error;
pub mod from_str;
pub mod macros;
pub mod find_variable;
mod add;
mod negate;
mod multiply;
mod invert;
mod isolate_variable;
pub mod number;
mod operations;

trait Operand: Display + Clone + PartialEq{
    fn solve(&self, variables: Option<&HashMap<String, f64>>) -> Result<f64, ExpressionError>;
    fn operand_count(&self) -> usize;
    fn children(&self) -> Vec<&Expression>;
    fn simplify(&self) -> Expression;
}

type Expr = Box<Expression>;
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Expression {
    /// Known mathematical constants, Like pi, e, etc
    Constant(Constant),
    /// A 64bit floating point number
    Number(Number),
    /// An unresolved variable, like x
    Variable(String),
    Add(Add),
    Multiply(Multiply),
    Power(Expr, Expr),
    Sqrt(Expr),
    /// Logarithm with base b of a
    Log(Expr, Expr),
    Sin(Expr),
    ArcSin(Expr),
    Cos(Expr),
    ArcCos(Expr),
    Tan(Expr),
    ArcTan(Expr),
    Ln(Expr),
    Abs(Expr),
    Negate(Negate),
    Invert(Invert),
}

impl Expression {
    pub fn solve(&self, variables: Option<&HashMap<String, f64>>) -> Result<f64, ExpressionError> {
        use Expression::*;
        Ok(match self {
            Number(a) => a.solve(variables)?,
            Constant(a) => a.solve(),
            Variable(a) => *variables
                .ok_or(ExpressionError::MissingVariable(a.clone()))?
                .get(a)
                .ok_or(ExpressionError::MissingVariable(a.clone()))?,
            Add(add) => add.solve(variables)?,
            Multiply(multiply) => multiply.solve(variables)?,
            Power(a, b) => a.solve(variables)?.powf(b.solve(variables)?),
            Sqrt(a) => a.solve(variables)?.sqrt(),
            Log(a, b) => a.solve(variables)?.log(b.solve(variables)?),
            Sin(a) => a.solve(variables)?.sin(),
            ArcSin(a) => a.solve(variables)?.asin(),
            Cos(a) => a.solve(variables)?.cos(),
            ArcCos(a) => a.solve(variables)?.acos(),
            Tan(a) => a.solve(variables)?.tan(),
            ArcTan(a) => a.solve(variables)?.atan(),
            Ln(a) => a.solve(variables)?.ln(),
            Abs(a) => a.solve(variables)?.abs(),
            Negate(negate) => negate.solve(variables)?,
            Invert(invert) => invert.solve(variables)?,
        })
    }

    pub fn simplify(&self) -> Expression {
        use Expression::*;
        match self {
            Negate(negate) => negate.simplify(),
            Add(add) => add.simplify(),
            Multiply(multiply) => multiply.simplify(),
            Invert(invert) => invert.simplify(),
            a => a.clone()
        }
    }

    pub fn children(&self) -> Vec<&Expression> {
        use Expression::*;
        match self {
            Add(expr) => expr.children(),
            Multiply(expr) => expr.children(),
            Negate(expr) => expr.children(),
            Invert(expr) => expr.children(),
            Power(a, b) => vec![a, b],
            Log(a, b) => vec![a, b],
            Sqrt(a) => vec![a],
            Sin(a) => vec![a],
            ArcSin(a) => vec![a],
            Cos(a) => vec![a],
            ArcCos(a) => vec![a],
            Tan(a) => vec![a],
            ArcTan(a) => vec![a],
            Ln(a) => vec![a],
            Abs(a) => vec![a],
            _ => vec![],
        }
    }

    pub fn contains_variable(&self, variable: &str) -> bool {
        use Expression::*;
        match self {
            Constant(_) |
            Number(_) => false,
            Variable(name) => name == variable,
            Add(add) => add.0.iter().any(|child| child.contains_variable(variable)),
            Multiply(multiply) => multiply.0.iter().any(|child| child.contains_variable(variable)),
            Power(left, right) |
            Log(left, right) => left.contains_variable(variable) || right.contains_variable(variable),
            Sqrt(inner) |
            Sin(inner) |
            ArcSin(inner) |
            Cos(inner) |
            ArcCos(inner) |
            Tan(inner) |
            ArcTan(inner) |
            Ln(inner) |
            Abs(inner) => inner.contains_variable(variable),
            Negate(negate) => negate.0.contains_variable(variable),
            Invert(invert) => invert.0.contains_variable(variable),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_basic_addition() {
        let input = add!(num!(1.0), num!(1.0),);
        let expected = 2.0;
        assert_eq!(input.solve(None).unwrap(), expected);
    }
    #[test]
    fn test_variable() {
        let input = mul!(var!("x"), inv!(num!(2.0)));
        let variables = HashMap::from([("x".to_string(), 2.0)] );
        assert_eq!(input.solve(Some(&variables)).unwrap(), 1.0);
    }
}
