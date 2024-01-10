use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter;
use crate::expression::display::parenthesize_if_of_type;
use crate::expression::{Expression, number, Operand};
use crate::expression::error::ExpressionError;
use crate::expression::multiply::Multiply;
use crate::{inv, mul, num};

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Invert(pub Box<Expression>);

impl Display for Invert{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        let a: String = parenthesize_if_of_type!(*self.0, Add(..) | Multiply(..) | Power(..) | Negate(..));
        write!(f, "1 / {}", a)
    }
}

impl Operand for Invert{
    fn solve(&self, variables: Option<&HashMap<String, f64>>) -> Result<f64, ExpressionError> {
        Ok( 1.0 / self.0.solve(variables)? )
    }

    fn operand_count(&self) -> usize {
        1
    }
    fn children(&self) -> Vec<&Expression> {
        vec![self.0.borrow()]
    }
    fn simplify(&self) -> Expression {
        use crate::expression::Expression::*;
        match self.0.simplify(){
            Invert(a) => *a.0,
            Number(number::Number(num)) => num!(1.0/num),
            a => inv!(a),
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::{inv, mul, num};
    #[test]
    fn inverted_multiply(){
        let expression = inv!(mul!(num!(2.0), inv!(num!(3.0))));
        assert_eq!(expression.simplify(), mul!(num!(3.0), inv!(num!(2.0))))
    }
}