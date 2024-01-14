use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::expression::{add, Expression, multiply};
use crate::{inv, neg};

impl Add<Expression> for Expression{
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
        use Expression::*;
        match (self, rhs){
            (Add(add), Add(add2)) => Add(add::Add(add.0.into_iter().chain(add2.0).collect())),
            (Add(add), rhs) => Add(add::Add(add.0.into_iter().chain(vec![rhs]).collect())),
            (lhs, Add(add)) => Add(add::Add(add.0.into_iter().chain(vec![lhs]).collect())),
            (lhs, rhs) => Add(add::Add(vec![lhs, rhs]))
        }
    }
}

impl Mul<Expression> for Expression{
    type Output = Expression;

    fn mul(self, rhs: Self) -> Self::Output {
        use Expression::*;
        match (self, rhs){
            (Multiply(mul), Multiply(mul2)) => Multiply(multiply::Multiply(mul.0.into_iter().chain(mul2.0).collect())),
            (Multiply(mul), rhs) => Multiply(multiply::Multiply(mul.0.into_iter().chain(vec![rhs]).collect())),
            (lhs, Multiply(mul)) => Multiply(multiply::Multiply(mul.0.into_iter().chain(vec![lhs]).collect())),
            (lhs, rhs) => Multiply(multiply::Multiply(vec![lhs, rhs]))
        }
    }
}

impl Div<Expression> for Expression{
    type Output = Expression;

    fn div(self, rhs: Self) -> Self::Output {
        use Expression::*;
        match (self, rhs){
            (Multiply(mul), rhs) => Multiply(multiply::Multiply(mul.0.into_iter().chain(vec![inv!(rhs)]).collect())),
            (lhs, rhs) => Multiply(multiply::Multiply(vec![lhs, inv!(rhs)]))
        }
    }
}

impl Sub<Expression> for Expression{
    type Output = Expression;

    fn sub(self, rhs: Expression) -> Self::Output {
        use Expression::*;
        match (self, rhs){
            (Add(add), Add(add2)) => Add(add::Add(add.0.into_iter().chain(add2.0.into_iter().map(|child| neg!(child))).collect())),
            (Add(add), rhs) => Add(add::Add(add.0.into_iter().chain(vec![neg!(rhs)]).collect())),
            (lhs, Add(add)) => Add(add::Add(add.0.into_iter().chain(vec![neg!(lhs)]).collect())),
            (lhs, rhs) => Add(add::Add(vec![lhs, neg!(rhs)]))
        }
    }
}

impl Neg for Expression{
    type Output = Expression;

    fn neg(self) -> Self::Output {
        neg!(self)
    }
}

impl From<f64> for Expression{
    fn from(num: f64) -> Self {
        Expression::Number(num.into())
    }
}

impl From<&str> for Expression{
    fn from(string: &str) -> Self {
        Expression::Variable(string.into())
    }
}