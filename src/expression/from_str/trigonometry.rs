use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::sequence::tuple;

use crate::expression::Expression;
use crate::expression::from_str::{expression, ws};

pub fn trigonometry(input: &str) -> IResult<&str, Expression> {
    let (input, (trig, _, inside, _)) = tuple((
        alt((
            tag("cos"),
            tag("sin"),
            tag("tan"),
            tag("arccos"),
            tag("arcsin"),
            tag("arctan"),
            tag("acos"),
            tag("asin"),
            tag("atan"),
            tag("cos^-1"),
            tag("sin^-1"),
            tag("tan^-1"),
        )),
        tag("("),
        ws(expression),
        tag(")")
    ))(input)?;

    match trig {
        "cos" => Ok((input, Expression::Cos(Box::new(inside)))),
        "sin" => Ok((input, Expression::Sin(Box::new(inside)))),
        "tan" => Ok((input, Expression::Tan(Box::new(inside)))),
        "arccos" | "acos" | "cos^-1" => Ok((input, Expression::ArcCos(Box::new(inside)))),
        "arcsin" | "asin" | "sin^-1" => Ok((input, Expression::ArcSin(Box::new(inside)))),
        "arctan" | "atan" | "tan^-1" => Ok((input, Expression::ArcTan(Box::new(inside)))),
        _ => unreachable!()
    }
}