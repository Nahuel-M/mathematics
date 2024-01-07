use core::fmt;
use std::fmt::{Display, Formatter};

use super::Expression;

macro_rules! parenthesize_if_of_type {
    ($expression:expr, $pattern:pat) => {{
        let mut text = $expression.to_string();
        if matches!($expression, $pattern){
            text = format!("({text})")
        }
        text
    }};
}

pub(crate) use parenthesize_if_of_type;

impl Display for Expression{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Expression::*;
        match self {
            Number(a) => write!(f, "{a}"),
            Constant(a) => write!(f, "{a}"),
            Variable(a) => write!(f, "{a}"),
            Add(add) => write!(f, "{add}"),
            Multiply(multiply) => {write!(f, "{multiply}")},
            Power(a, b) => {
                let a = parenthesize_if_of_type!(**a, Add(..) | Multiply(..));
                let b = parenthesize_if_of_type!(**b, Add(..) | Multiply(..));
                write!(f, "{a} ^ {b}")
            }
            Log(a, b) => write!(f, "log_{a}({b})"),
            Ln(a) => write!(f, "ln({a})"),
            Sin(a) => write!(f, "sin({a})"),
            Cos(a) => write!(f, "cos({a})"),
            Tan(a) => write!(f, "tan({a})"),
            ArcSin(a) => write!(f, "arcsin({a})"),
            ArcCos(a) => write!(f, "arccos({a})"),
            ArcTan(a) => write!(f, "arctan({a})"),
            Sqrt(a) => write!(f, "sqrt({a})"),
            Abs(a) => write!(f, "abs({a})"),
            Negate(negate) => { write!(f, "{negate}") }
            Invert(invert) => { write!(f, "1/({invert})") }
        }
    }
}