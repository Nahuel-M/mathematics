use std::collections::HashMap;
use std::fmt::Display;
use std::hash;
use std::hash::Hash;
use std::ops::Neg;
use crate::expression::error::ExpressionError;
use crate::expression::{Expression, Operand};
use crate::num;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Number(pub f64);

impl Operand for Number{
    fn solve(&self, _variables: Option<&HashMap<String, f64>>) -> Result<f64, ExpressionError> {
        Ok(self.0)
    }

    fn operand_count(&self) -> usize {
        0
    }

    fn children(&self) -> Vec<&Expression> {
        vec![]
    }

    fn simplify(&self) -> Expression {
       num!(self.0)
    }
}

impl Hash for Number {
    fn hash<H>(&self, state: &mut H)
        where
            H: hash::Hasher,
    {
        self.0.to_bits().hash(state)
    }
}

impl Eq for Number {}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.fract() == 0.0 {
            write!(f, "{}", self.0 as i64)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl Neg for Number{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::ops::Add for Number{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Number{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul for Number{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl std::ops::Div for Number{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl std::ops::Rem for Number{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl std::ops::AddAssign for Number{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::SubAssign for Number{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::MulAssign for Number{
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl std::ops::DivAssign for Number{
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl std::ops::RemAssign for Number{
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl From<f64> for Number{
    fn from(num: f64) -> Self {
        Self(num)
    }
}
// Check inequality with f64
impl PartialEq<f64> for Number{
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Number> for f64{
    fn eq(&self, other: &Number) -> bool {
        *self == other.0
    }
}

impl PartialOrd<f64> for Number{
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Number> for f64{
    fn partial_cmp(&self, other: &Number) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}