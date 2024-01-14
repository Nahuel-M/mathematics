use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::expression::error::ExpressionError;
use crate::expression::{Expression, number, Operand};
use crate::expression::display::parenthesize_if_of_type;
use crate::num;


#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Negate(pub Box<Expression>);

impl Operand for Negate{
    fn solve(&self, variables: Option<&HashMap<String, f64>>) -> Result<f64, ExpressionError> {
        Ok(-self.0.solve(variables)?)
    }
    fn operand_count(&self) -> usize {
        1
    }

    fn children(&self) -> Vec<&Expression> {
        vec![self.0.borrow()]
    }

    fn simplify(&self) -> Expression {
        use crate::expression::Expression::*;
        match self.0.simplify() {
            Number(number::Number(a)) => num!(-a),
            Negate(a) => *a.0, // Double negative
            a => a,
        }
    }
}


impl Display for Negate{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        let a: String = parenthesize_if_of_type!(*self.0, Add(..) | Multiply(..) | Power(..) | Negate(..));
        write!(f, "-{a}")
    }
}