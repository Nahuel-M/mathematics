use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::iter;
use crate::expression::error::ExpressionError;
use crate::expression::{Expression, Operand};
use crate::expression::display::parenthesize_if_of_type;
use crate::{add, multiply, number};
use crate::expression::multiply::Multiply;

#[derive(Debug, Clone, PartialEq)]
pub struct Add{
    children: Vec<Expression>
}

impl Operand for Add{
    fn solve(&self, variables: Option<&HashMap<String, f64>>) -> Result<f64, ExpressionError> {
        let results: Result<Vec<f64>, _> = self.children.iter()
            .map(|child| child.solve(variables))
            .collect();
        Ok(results?.iter().sum())
    }

    fn operand_count(&self) -> usize{
        2
    }

    fn children(&self) -> impl Iterator<Item = &Expression> {
        self.children.iter()
    }

    fn simplify(&self) -> Expression {
        use Expression::*;
        let mut children: Vec<Expression> = self.children.iter()
            .map(|child| child.simplify())
            .collect();

        let number_sum = children
            .iter()
            .filter_map(|child| match child{
                Number(a) => Some(*a),
                _ => None,
            })
            .sum();

        children.retain(|child| !matches!(child, Number(_)));


        return add!(self)
        // let a = &*self.left;
        // let b = &*self.right;
        // use Expression::*;
        // match (a.simplify(), b.simplify()) {
        //     (Number(a), Number(b)) => number!(a + b),
        //     (a, Number(b)) if b == 0.0 => a,
        //     (Number(a), b) if a == 0.0 => b,
        //     (a, Negate(b)) if a == *b.inner => { number!(0.0) },
        //     (Negate(a), b) if *a.inner == b => { number!(0.0) },
        //     (Multiply(a), Multiply(b)) => simplify_added_multiply(a, b),
        //     (a, b) if a == b => multiply!(number!(2.0), a),
        //     (a,b) => add!(a, b),
        // }
    }
}

fn simplify_added_multiply(left: Multiply, right: Multiply) -> Expression{
    use Expression::*;
    if left.left == right.left{
        multiply!(*left.left, add!(*left.right, *right.right))
    } else if left.left == right.right{
        multiply!(*left.left, add!(*left.right, *right.left))
    } else if left.right == right.left{
        multiply!(*left.right, add!(*left.left, *right.right))
    } else if left.right == right.right{
        multiply!(*left.right, add!(*left.left, *right.left))
    } else {
        add!(Multiply(left), Multiply(right))
    }
}

impl Display for Add{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        let a = parenthesize_if_of_type!(*self.left, Multiply(..) | Power(..) | Negate(..));
        if let Negate(right) = *self.right.clone(){
        let b = parenthesize_if_of_type!(*right.inner, Add(..) | Multiply(..) | Power(..) | Negate(..));
            write!(f, "{a} - {b}")
        } else {
            let b = parenthesize_if_of_type!(*self.right, Multiply(..) | Power(..) | Negate(..));
            write!(f, "{a} + {b}")
        }
    }
}

#[cfg(test)]
pub mod test{
    use crate::{add, number, negate};
    use super::*;

    #[test]
    fn test_display(){
        let expr1 = add!(number!(1.0), number!(2.0));
        assert_eq!(format!("{}", expr1), "1 + 2");
        let expr2 = add!(add!(number!(1.0), number!(2.0)), number!(3.0));
        assert_eq!(format!("{}", expr2), "1 + 2 + 3");
        let expr3 = add!(number!(1.0), add!(number!(2.0), negate!(number!(3.0))));
        assert_eq!(format!("{}", expr3), "1 + 2 - 3");
        let expr4 = add!(number!(1.0), negate!(add!(number!(2.0), number!(3.0))));
        assert_eq!(format!("{}", expr4), "1 - (2 + 3)");
    }
}