use std::collections::HashMap;

use self::{constant::Constant, error::ExpressionError};

pub mod constant;
pub mod error;
pub mod operation;
pub mod display;

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
    pub fn parse_latex_formula(input: &String) -> Expression {
        Expression::Number(0.0)
    }

    pub fn solve(
        &self,
        variables: Option<&HashMap<String, f64>>,
    ) -> Result<f64, ExpressionError> {
        use Expression::*;
        match self {
            Number(a) => Ok(*a),
            Constant(a) => Ok(a.solve()),
            Variable(a) => variables
                .ok_or(ExpressionError::MissingVariable(a.clone()))?
                .get(a)
                .cloned()
                .ok_or(ExpressionError::MissingVariable(a.clone())),
            Add(a, b) => Ok(a.solve(variables)? + b.solve(variables)?),
            Subtract(a, b) => Ok(a.solve(variables)? - b.solve(variables)?),
            Multiply(a, b) => Ok(a.solve(variables)? * b.solve(variables)?),
            Divide(a, b) => Ok(a.solve(variables)? / b.solve(variables)?),
            Power(a, b) => Ok(a.solve(variables)?.powf(b.solve(variables)?)),
            Sqrt(a) => Ok(a.solve(variables)?.sqrt()),
            Log(a, b) => Ok(a.solve(variables)?.log(b.solve(variables)?)),
            Sin(a) => Ok(a.solve(variables)?.sin()),
            ArcSin(a) => Ok(a.solve(variables)?.asin()),
            Cos(a) => Ok(a.solve(variables)?.cos()),
            ArcCos(a) => Ok(a.solve(variables)?.acos()),
            Tan(a) => Ok(a.solve(variables)?.tan()),
            ArcTan(a) => Ok(a.solve(variables)?.atan()),
            Exp(a) => Ok(a.solve(variables)?.exp()),
            Ln(a) => Ok(a.solve(variables)?.ln()),
            Abs(a) => Ok(a.solve(variables)?.abs()),
            Negate(a) => Ok(-a.solve(variables)?),
        }
    }

    pub fn simplify(&self) -> Expression {
        match self {
            Expression::Number(a) => self.clone(),
            Expression::Constant(a) => self.clone(),
            Expression::Variable(a) => self.clone(),
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Expression::*;

    #[test]
    fn test_parse_latex_formula() {
        let input = String::from("x^2 + 2x + 1");
        let expected = Add(
            Box::new(Power(
                Box::new(Variable(String::from("x"))),
                Box::new(Number(2.0)),
            )),
            Box::new(Add(
                Box::new(Multiply(
                    Box::new(Number(2.0)),
                    Box::new(Variable(String::from("x"))),
                )),
                Box::new(Number(1.0)),
            )),
        );
        assert_eq!(Expression::parse_latex_formula(&input), expected);
    }
    #[test]
    fn test_basic_addition() {
        let input = Add(
            Box::new(Expression::Number(1.0)),
            Box::new(Expression::Number(1.0)),
        );
        let expected = 2.0;
        assert_eq!(input.solve(None).unwrap(), expected);
    }
    #[test]
    fn test_variable() {
        use Expression::Variable;
        let input = Divide(
            Box::new(Variable(String::from("x"))),
            Box::new(Expression::Number(2.0)),
        );
        let mut variables = HashMap::new();
        variables.insert(String::from("x"), 2.0);
        let expected = 1.0;
        assert_eq!(input.solve(Some(&variables)).unwrap(), expected);
    }
}
