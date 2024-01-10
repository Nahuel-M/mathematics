use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Constant {
    Pi,
    E,
}

impl Constant {
    pub fn solve(&self) -> f64 {
        match self {
            Constant::Pi => std::f64::consts::PI,
            Constant::E => std::f64::consts::E,
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Pi => write!(f, "π"),
            Constant::E => write!(f, "e"),
        }
    }
}