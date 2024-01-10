use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::{mul, num};
use crate::expression::{add, Expression, multiply, negate, number, Operand};
use crate::expression::display::parenthesize_if_of_type;
use crate::expression::error::ExpressionError;
use crate::utils::insert_or_add::InsertOrAdd;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Add(pub Vec<Expression>);

impl Operand for Add {
    fn solve(&self, variables: Option<&HashMap<String, f64>>) -> Result<f64, ExpressionError> {
        let results: Result<Vec<f64>, _> = self.0.iter()
            .map(|child| child.solve(variables))
            .collect();
        Ok(results?.iter().sum())
    }

    fn operand_count(&self) -> usize {
        2
    }

    fn children(&self) -> Vec<&Expression> {
        self.0.iter().collect()
    }

    fn simplify(&self) -> Expression {
        let mut number_sum: f64 = 0.0;
        let mut new_children: Vec<Expression> = Vec::new();
        use Expression::*;
        let children: Vec<Expression> = self.0.iter()
            .flat_map(|child| {
                match child.simplify(){
                    Add(add::Add(children)) => children,
                    other => vec![other]
                }})
            .collect();

        let mut additions: HashMap<Expression, f64> = HashMap::new();
        for child in children {
            match &child {
                Number(number::Number(num)) => number_sum += num,
                Negate(negate::Negate(negate)) => additions.insert_or_add(*negate.clone(), -1.0),
                Multiply(multiply::Multiply(children)) => {
                    let numeric_multiple = children.iter().find(|child| matches!(child, Number(..)));
                    if let Some(Number(number::Number(num))) = numeric_multiple {
                        let remainder = children.iter().filter(|child| !matches!(child, Number(..))).cloned().collect_vec();
                        additions.insert_or_add(Multiply(multiply::Multiply(remainder)), *num);
                    } else {
                        additions.insert_or_add(child, 1.0);
                    }
                }
                _ => additions.insert_or_add(child, 1.0)
            }
        }
        for (addition, count) in additions.into_iter(){
            if count == 1.{
                new_children.push(addition);
            } else if count != 0.{
                new_children.push(mul!(num!(count), addition))
            }
        }

        if number_sum != 0.0 {
            new_children.push(num!(number_sum));
        }

        if new_children.len() == 1{
            return new_children[0].clone();
        }

        Add(Self(new_children))
    }
}


impl Display for Add {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        if self.0.is_empty() {
            return write!(f, "0");
        }
        write!(f, "{}", self.0[0])?;

        if self.0.len() == 1 {
            return Ok(());
        }

        for child in &self.0[1..] {
            if let Negate(negate) = child {
                write!(f, " - {}", parenthesize_if_of_type!(*negate.0, Add(..)))?;
            } else if let Number(number) = *child{
                if number < 0.0{
                    write!(f, " - {}", -number)?;
                    continue;
                }
                write!(f, " + {}", child)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use crate::{add, neg, num, var};

    use super::*;

    #[test]
    fn test_display() {
        let expr1 = add!(num!(1), num!(2));
        assert_eq!(format!("{expr1}"), "1 + 2");
        let expr2 = add!(num!(1), num!(2), num!(3));
        assert_eq!(format!("{expr2}"), "1 + 2 + 3");
        let expr3 = add!(num!(1), num!(2), neg!(num!(3)));
        assert_eq!(format!("{expr3}"), "1 + 2 - 3");
        let expr4 = add!(num!(1), neg!(add!(num!(2), num!(-3))));
        assert_eq!(format!("{expr4}"), "1 - (2 - 3)");
    }

    #[test]
    fn test_simplification(){
        let expr1 = add!(add!(num!(1), var!("a")), var!("b"), num!(-2));
        assert_eq!(expr1.simplify(), add!(var!("a"), var!("b"), num!(-1)));
        let expr2 = add!(
            mul!(num!(1), var!("a")),
            var!("b"),
            num!(-2),
            num!(3),
            mul!(num!(5), var!("a"))
        );
        assert_eq!(expr2.simplify(), add!(
            mul!(num!(6), var!("a")),
            var!("b"),
            num!(1),
        ));
    }
}