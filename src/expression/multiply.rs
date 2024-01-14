use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use rustc_hash::FxHashMap;
use crate::expression::{Expression, invert, multiply, number, Operand};
use crate::expression::display::parenthesize_if_of_type;
use crate::expression::error::ExpressionError;
use crate::{inv, num, pow};
use crate::utils::insert_or_add::InsertOrAdd;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Multiply(pub Vec<Expression>);

impl Operand for Multiply{
    fn solve(&self, variables: Option<&HashMap<String, f64>>) -> Result<f64, ExpressionError> {
        let results: Result<Vec<f64>, _> = self.0.iter()
            .map(|child| child.solve(variables))
            .collect();
        Ok(results?.iter().product())
    }

    fn operand_count(&self) -> usize{ 2 }

    fn children(&self) -> Vec<&Expression> {
        self.0.iter().collect()
    }
    fn simplify(&self) -> Expression {
        let mut number_product: f64 = 1.0;
        let mut new_children: Vec<Expression> = Vec::new();
        use Expression::*;
        let children: Vec<Expression> = self.0.iter()
            .flat_map(|child| {
                match child.simplify(){
                    Multiply(multiply::Multiply(children)) => children, // Merge inner multiplies
                    other => vec![other]
                }})
            .collect();

        let mut multiplications: FxHashMap<Expression, f64> = FxHashMap::default();
        for child in children {
            match &child {
                Number(number::Number(num)) => {
                    if num == &0.0 {
                        return num!(0.0);
                    }
                    number_product *= num;
                },
                Invert(invert::Invert(invert)) => multiplications.insert_or_add(*invert.clone(), -1.0),
                Power(ref base, ref exponent) => {
                    if let Number(number::Number(num)) = exponent.as_ref() {
                        multiplications.insert_or_add(*base.clone(), *num);
                    } else {
                        multiplications.insert_or_add(child, 1.0);
                    }
                }
                _ => multiplications.insert_or_add(child, 1.0)
            }
        }
        for (multiplication, count) in multiplications.into_iter(){
            if count == 1. {
                new_children.push(multiplication);
            } else if count == -1.{
                new_children.push(inv!(multiplication));
            } else if count != 0.{
                new_children.push(pow!(multiplication, num!(count)));
            }
        }

        if number_product != 1.0 {
            new_children.push(num!(number_product));
        }

        if new_children.len() == 1{
            return new_children[0].clone();
        }

        Multiply(Self(new_children))
    }
}

impl Multiply{
    pub fn not_containing_variable(&self, variable: &str) -> Vec<Expression>{
        self.0.iter()
            .filter(|child| !child.contains_variable(variable))
            .cloned()
            .collect()
    }
}

impl Display for Multiply{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        if self.0.is_empty(){
            return write!(f, "0");
        }
        write!(f, "{}", self.0[0])?;

        if self.0.len() == 1{
            return Ok(());
        }

        for child in &self.0[1..]{
            if matches!(child, Invert(..)){
                write!(f, " / {}", parenthesize_if_of_type!(child, Add(..)))?;
            } else {
                write!(f, " * {}", parenthesize_if_of_type!(child, Add(..)))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::{num, var};
    #[test]
    fn test_simplification(){
        let expression = num!(1.0) * var!("a");
        assert_eq!(expression.simplify(), var!("a"));
        let expression = num!(2.0) / num!(3.0);
        assert_eq!(expression.simplify(), num!(2.0/3.0));
        let expression = num!(5) * var!("a") * pow!(var!("a"), num!(5)) * num!(4) / var!("a");
        assert_eq!(expression.simplify(), pow!(var!("a"), num!(5)) * num!(20));
    }
}