use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Mul;
use crate::expression::{Expression, Operand};
use crate::expression::display::parenthesize_if_of_type;
use crate::expression::error::ExpressionError;
use crate::{multiply, number, power};

#[derive(Clone, Debug, PartialEq)]
pub struct Multiply{
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl Operand for Multiply{
    fn solve(&self, variables: Option<&HashMap<String, f64>>) -> Result<f64, ExpressionError> {
        Ok(self.left.solve(variables)? * self.right.solve(variables)?)
    }

    fn operand_count(&self) -> usize{ 2 }

    fn children(&self) -> impl Iterator<Item=&Expression> {
        [&*self.left, &*self.right].into_iter()
    }
    fn simplify(&self) -> Expression {
        let a = &*self.left;
        let b = &*self.right;
        use Expression::*;
        match (a.simplify(), b.simplify()) {
            (Number(a), Number(b)) => number!(a * b),
            (_a, Number(b)) if b == 0.0 => number!(0.0),
            (Number(a), _b) if a == 0.0 => number!(0.0),
            (a, Number(b)) if b == 1.0 => a,
            (Number(a), b) if a == 1.0 => b,
            (a, b) if a == b => power!(a, number!(2.0)),
            (a, b) => multiply!(a, b)
        }
    }
}

impl Display for Multiply{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        let a: String = parenthesize_if_of_type!(*self.left, Add(..) | Multiply(..) | Power(..) | Negate(..));
        if let Expression::Invert(right) = *self.right.clone(){
            let b: String = parenthesize_if_of_type!(*right.inner, Add(..) | Multiply(..) | Power(..) | Negate(..));
            write!(f, "{} / {}", a, b)
        } else {
            let b: String = parenthesize_if_of_type!(*self.right, Add(..) | Multiply(..) | Power(..) | Negate(..));
            write!(f, "{} + {}", a, b)
        }
    }
}