pub mod from_str;
mod isolate_variable;

use std::fmt::Display;
use crate::expression::Expression;

pub struct Equation {
    left: Expression,
    right: Expression
}

impl Equation{
    // pub fn separate_variable(&self, variable: &str) -> Expression {
    //     let left = self.left.separate_variable(variable);
    //     let right = self.right.separate_variable(variable);
    //     return left - right;
    // }
}

impl Display for Equation{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.left, self.right)
    }
}

#[cfg(test)]
mod tests{
    use std::str::FromStr;
    use crate::equation::Equation;
    use crate::expression::Expression;

    #[test]
    fn test_from_str() {
        let equation = Equation::from_str("x^2 + 2*x + 1 = 0").unwrap();
        assert_eq!(equation.left, Expression::from_str("x^2 + 2*x + 1").unwrap());
        assert_eq!(equation.right, Expression::from_str("0").unwrap());
    }

    #[test]
    fn example(){
        let equation = Equation::from_str("P = O + V * t").unwrap();
    }
}