use std::collections::HashMap;
use crate::expression::simplify::*;

use self::{constant::Constant, error::ExpressionError};

pub mod constant;
pub mod error;
pub mod display;
pub mod macros;
pub mod simplify;
pub mod from_str;

type Expr = Box<Expression>;
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Constant(Constant),
    Number(f64),
    Variable(String),
    Add(Expr, Expr),
    Subtract(Expr, Expr),
    Multiply(Expr, Expr),
    Divide(Expr, Expr),
    Power(Expr, Expr),
    Sqrt(Expr),
    Log(Expr, Expr),
    Sin(Expr),
    ArcSin(Expr),
    Cos(Expr),
    ArcCos(Expr),
    Tan(Expr),
    ArcTan(Expr),
    Exp(Expr),
    Ln(Expr),
    Abs(Expr),
    Negate(Expr),
}

impl Expression {
    pub fn parse_latex_formula(_input: &str) -> Expression {
        Expression::Number(0.0)
    }

    pub fn solve(
        &self,
        variables: Option<&HashMap<String, f64>>,
    ) -> Result<f64, ExpressionError> {
        use Expression::*;
        Ok(match self {
            Number(a) => *a,
            Constant(a) => a.solve(),
            Variable(a) => variables
                .ok_or(ExpressionError::MissingVariable(a.clone()))?
                .get(a)
                .cloned()
                .ok_or(ExpressionError::MissingVariable(a.clone()))?,
            Add(a, b) => a.solve(variables)? + b.solve(variables)?,
            Subtract(a, b) => a.solve(variables)? - b.solve(variables)?,
            Multiply(a, b) => a.solve(variables)? * b.solve(variables)?,
            Divide(a, b) => a.solve(variables)? / b.solve(variables)?,
            Power(a, b) => a.solve(variables)?.powf(b.solve(variables)?),
            Sqrt(a) => a.solve(variables)?.sqrt(),
            Log(a, b) => a.solve(variables)?.log(b.solve(variables)?),
            Sin(a) => a.solve(variables)?.sin(),
            ArcSin(a) => a.solve(variables)?.asin(),
            Cos(a) => a.solve(variables)?.cos(),
            ArcCos(a) => a.solve(variables)?.acos(),
            Tan(a) => a.solve(variables)?.tan(),
            ArcTan(a) => a.solve(variables)?.atan(),
            Exp(a) => a.solve(variables)?.exp(),
            Ln(a) => a.solve(variables)?.ln(),
            Abs(a) => a.solve(variables)?.abs(),
            Negate(a) => -a.solve(variables)?,
        })
    }

    pub fn simplify(&self) -> Expression {
        use Expression::*;
        match self {
            Number(_) => self.clone(),
            Constant(_) => self.clone(),
            Variable(_) => self.clone(),
            Negate(a) => match a.simplify() {
                Number(a) => Number(-a),
                Negate(a) => *a, // Double negative
                _ => Negate(a.clone()),
            },
            Add(a, b) => simplify_add(a, b),
            Subtract(a, b) => simplify_subtract(a, b),
            Multiply(a, b) => simplify_multiplication(a, b),
            Divide(a, b) => simplify_division(a, b),
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test]
    fn test_parse_latex_formula() {
        let input = String::from("x^2 + 2x + 1");
        let expected = add!(
            power!(
                variable!("x"),
                number!(2.0),
            ),
            add!(
                multiply!(
                    number!(2.0),
                    variable!("x")
                ),
                number!(1.0)
            ),
        );
        assert_eq!(Expression::parse_latex_formula(&input), expected);
    }
    #[test]
    fn test_basic_addition() {
        let input = add!(
            number!(1.0),
            number!(1.0),
        );
        let expected = 2.0;
        assert_eq!(input.solve(None).unwrap(), expected);
    }
    #[test]
    fn test_variable() {
        let input = divide!(
            variable!("x"),
            number!(2.0),
        );
        let mut variables = HashMap::new();
        variables.insert(String::from("x"), 2.0);
        let expected = 1.0;
        assert_eq!(input.solve(Some(&variables)).unwrap(), expected);
    }
}
