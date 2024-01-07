use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::sequence::{tuple};
use crate::expression::Expression;
use crate::expression::from_str::{expression, ws};
use crate::{abs, ln, sqrt};

pub(crate) fn singletons(input: &str) -> IResult<&str, Expression> {
    let (input, (operation, _, inside, _)) = tuple((
        alt((
            tag("sqrt"),
            tag("ln"),
            tag("abs"),
        )),
        tag("("),
        ws(expression),
        tag(")")
    ))(input)?;

    let expression = match operation {
        "sqrt" => sqrt!(inside),
        "ln" => ln!(inside),
        "abs" => abs!(inside),
        _ => unreachable!()
    };

    return Ok((input, expression));
}

