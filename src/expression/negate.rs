use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::iter;
use crate::expression::error::ExpressionError;
use crate::expression::{Expression, Operand};
use crate::expression::display::parenthesize_if_of_type;
use crate::{add, negate, number};
use crate::expression::add::Add;

#[derive(Debug, Clone, PartialEq)]
pub struct Negate{
    pub inner: Box<Expression>
}

impl Operand for Negate{
    fn solve(&self, variables: Option<&HashMap<String, f64>>) -> Result<f64, ExpressionError> {
        Ok(-self.inner.solve(variables)?)
    }
    fn operand_count(&self) -> usize {
        1
    }

    fn children(&self) -> impl Iterator<Item=&Expression> {
        iter::once(&*self.inner)
    }

    fn simplify(&self) -> Expression {
        use crate::expression::Expression as Exp;
        match self.inner.simplify() {
            Exp::Number(a) => number!(-a),
            Exp::Add(Add{left, right}) => simplify_negated_add(&*left, &*right),
            Exp::Negate(a) => *a.inner, // Double negative
            a => a,
        }
    }
}

fn simplify_negated_add(left: &Expression, right: &Expression) -> Expression{
    if let Expression::Negate(left) = left{
        return add!(*left.inner.clone(), negate!(right.clone())); // -(-a + b) = a - b
    } else if let Expression::Negate(right) = right{
        return add!(*right.inner.clone(), negate!(left.clone())); // -(a - b) = b - a
    }
    negate!(add!(left.clone(), right.clone()))
}

impl Display for Negate{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        let a: String = parenthesize_if_of_type!(*self.inner, Add(..) | Multiply(..) | Power(..) | Negate(..));
        write!(f, "-{a}")
    }
}