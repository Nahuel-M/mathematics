use std::collections::HashMap;
use std::fmt::Display;
use std::iter;
use crate::expression::display::parenthesize_if_of_type;
use crate::expression::{Expression, Operand};
use crate::expression::error::ExpressionError;
use crate::expression::multiply::Multiply;
use crate::{invert, multiply};

#[derive(Clone, Debug, PartialEq)]
pub struct Invert{
    pub inner: Box<Expression>,
}

impl Display for Invert{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        let a: String = parenthesize_if_of_type!(*self.inner, Add(..) | Multiply(..) | Power(..) | Negate(..));
        write!(f, "1/{}", a)
    }
}

impl Operand for Invert{
    fn solve(&self, variables: Option<&HashMap<String, f64>>) -> Result<f64, ExpressionError> {
        Ok( 1.0 / self.inner.solve(variables)? )
    }

    fn operand_count(&self) -> usize {
        1
    }
    fn children(&self) -> impl Iterator<Item=&Expression> {
        iter::once(&*self.inner)
    }
    fn simplify(&self) -> Expression {
        use crate::expression::Expression::Invert;
        match self.inner.simplify(){
            Invert(a) => *a.inner,
            Expression::Multiply(Multiply{left, right}) => match_inverted_multiply(&*left, &*right),
            a => invert!(a),
        }
    }
}

fn match_inverted_multiply(left: &Expression, right: &Expression) -> Expression{
    if let Expression::Invert(left) = left{
        return multiply!(*left.inner.clone(), invert!(right.clone())); // 1/(1/a*b) = a/b
    } else if let Expression::Invert(right) = right{
        return multiply!(*right.inner.clone(), invert!(left.clone())); // 1/(a * (1/b)) = b/a
    }
    invert!(multiply!(left.clone(), right.clone()))
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::{invert, multiply, number};
    #[test]
    fn inverted_multiply(){
        let expression = invert!(multiply!(number!(2.0), invert!(number!(3.0))));
        assert_eq!(expression.simplify(), multiply!(number!(3.0), invert!(number!(2.0))))
    }
}