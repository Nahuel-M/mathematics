use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::error::{Error, ParseError};
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, tuple};

use Expression::*;

use crate::{add, divide, multiply, power, negate, invert};
use crate::expression::constant::Constant::*;
use crate::expression::Expression;
use crate::expression::from_str::singletons::singletons;
use crate::expression::from_str::trigonometry::trigonometry;

mod trigonometry;
mod singletons;

impl Expression {
    pub fn from_str(input: &str) -> Result<Expression, nom::Err<Error<&str>>> {
        expression(input).map(|(_, expression)| expression)
    }
}

fn singleton(input: &str) -> IResult<&str, Expression> {
    alt((
        bracketed,
        trigonometry,
        singletons,
        constant,
        number,
        variable,
    ))(input)
}

fn higher_than_power(input: &str) -> IResult<&str, Expression> {
    alt((
        singleton,
    ))(input)
}

fn higher_than_multiplicative(input: &str) -> IResult<&str, Expression> {
    alt((
        power,
        higher_than_power,
    ))(input)
}

fn higher_than_add(input: &str) -> IResult<&str, Expression> {
    alt((
        multiplicative,
        higher_than_multiplicative,
    ))(input)
}

pub(crate) fn expression(input: &str) -> IResult<&str, Expression> {
    let (mut remaining_input, mut expression) = singleton(input)?;
    if remaining_input.is_empty() {
        return Ok((remaining_input, expression));
    }

    while let Ok((input, (operation, next_expression))) = alt((
        tuple((ws(alt((tag("+"), tag("-")))), higher_than_add)),
        tuple((ws(alt((tag("*"), tag("/")))), higher_than_multiplicative)),
        tuple((ws(tag("^")), higher_than_power)),
    ))(remaining_input) {
        remaining_input = input;
        expression = match operation {
            "+" => add!(expression, next_expression),
            "-" => add!(expression, negate!(next_expression)),
            "*" => multiply!(expression, next_expression),
            "/" => multiply!(expression, invert!(next_expression)),
            "^" => power!(expression, next_expression),
            _ => unreachable!()
        };
    }
    return Ok((remaining_input, expression));
}

fn multiplicative(input: &str) -> IResult<&str, Expression> {
    let (mut remaining_input, mut expression) = singleton(input)?;
    if remaining_input.is_empty() {
        return Ok((remaining_input, expression));
    }

    while let Ok((input, (operation, next_expression))) = alt((
        tuple((ws(tag("*")), higher_than_multiplicative)),
        tuple((ws(tag("/")), higher_than_multiplicative)),
        tuple((ws(tag("^")), higher_than_power)),
    ))(remaining_input) {
        remaining_input = input;
        expression = match operation {
            "*" => multiply!(expression, next_expression),
            "/" => multiply!(expression, invert!(next_expression)),
            "^" => power!(expression, next_expression),
            _ => unreachable!()
        };
    }
    return Ok((remaining_input, expression));
}

fn power(input: &str) -> IResult<&str, Expression> {
    let (mut remaining_input, mut expression) = singleton(input)?;
    if remaining_input.is_empty() {
        return Ok((remaining_input, expression));
    }
    while let Ok((input, next_expression)) = preceded(ws(tag("^")), higher_than_power)(remaining_input) {
        remaining_input = input;
        expression =  power!(expression, next_expression);
    }
    return Ok((remaining_input, expression));
}


fn constant(input: &str) -> IResult<&str, Expression> {
    alt((
        map(tag("pi"), |_| Constant(Pi)),
        map(tag("e"), |_| Constant(E)),
    ))(input)
}

fn variable(input: &str) -> IResult<&str, Expression> {
    take_while(|c: char| c.is_alphanumeric() || c == '_')(input).map(|(input, variable)| (input, Variable(variable.to_string())))
}

fn number(input: &str) -> IResult<&str, Expression> {
    double(input).map(|(input, number)| (input, Number(number)))
}

fn bracketed(input: &str) -> IResult<&str, Expression> {
    delimited(ws(tag("(")), expression, ws(tag(")")))(input)
}

pub fn ws<'a, F, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where F: Parser<&'a str, O, E>, {
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod tests {
    use crate::{add, divide, multiply, number, power, negate, variable};
    use super::*;

    #[test]
    fn test_constant() {
        assert_eq!(constant("pi"), Ok(("", Constant(Pi))));
        assert_eq!(constant("e"), Ok(("", Constant(E))));
    }

    #[test]
    fn test_number() {
        assert_eq!(number("1"), Ok(("", Number(1.0))));
        assert_eq!(number("1.0"), Ok(("", Number(1.0))));
        assert_eq!(number("1.0e-1"), Ok(("", Number(0.1))));
    }

    #[test]
    fn test_add() {
        assert_eq!(expression("1 + 2"), Ok(("", add!(number!(1.0), number!(2.0)))));
        assert_eq!(expression("1 + 2 + 3"), Ok(("", add!(add!(number!(1.0), number!(2.0)),number!(3.0)))));
        assert_eq!(expression("1 + 2 + 3 + 4"), Ok(("", add!(add!(add!(number!(1.0), number!(2.0)), number!(3.0)),number!(4.0)))));
        assert_eq!(expression("1 + 2 - 3"), Ok(("", add!(add!(number!(1.0), number!(2.0)), negate!(number!(3.0))))));
        assert_eq!(expression("1 - 2 + 3"), Ok(("", add!(add!(number!(1.0), negate!(number!(2.0))), number!(3.0)))));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(expression("1 * 2"), Ok(("", multiply!(number!(1.0), number!(2.0)))));
        assert_eq!(expression("1 * 2 * 3"), Ok(("", multiply!(multiply!(number!(1.0), number!(2.0)), number!(3.0)))));
        assert_eq!(expression("1 * 2 * 3 * 4"), Ok(("", multiply!(multiply!(multiply!(number!(1.0), number!(2.0)), number!(3.0)), number!(4.0)))));
        assert_eq!(expression("1 * 2 / 3"), Ok(("", multiply!(multiply!(number!(1.0), number!(2.0)), invert!(number!(3.0))))));
        assert_eq!(expression("1 / 2 * 3"), Ok(("", multiply!(multiply!(number!(1.0), invert!(number!(2.0))), number!(3.0)))));
    }

    #[test]
    fn test_combinations() {
        assert_eq!(expression("1 + 2 * 3"), Ok(("", add!(number!(1.0), multiply!(number!(2.0), number!(3.0))))));
        assert_eq!(expression("1 * 2 + 3"), Ok(("", add!(multiply!(number!(1.0), number!(2.0)), number!(3.0)))));
        assert_eq!(expression("1 * 2 + 3 * 4"), Ok(("", add!(multiply!(number!(1.0), number!(2.0)), multiply!(number!(3.0), number!(4.0))))));
        assert_eq!(expression("1 + 2 / 3"), Ok(("", add!(number!(1.0), multiply!(number!(2.0), invert!(number!(3.0)))))));
        assert_eq!(expression("1 / 2 + 3"), Ok(("", add!(multiply!(number!(1.0), invert!(number!(2.0))), number!(3.0)))));
    }

    #[test]
    fn test_power() {
        assert_eq!(expression("1 ^ 2"), Ok(("", power!(number!(1.0), number!(2.0)))));
        assert_eq!(expression("1 ^ 2 ^ 3"), Ok(("", power!(power!(number!(1.0), number!(2.0)), number!(3.0)))));
        assert_eq!(expression("1 ^ 2 ^ 3 ^ 4"), Ok(("", power!(power!(power!(number!(1.0), number!(2.0)), number!(3.0)), number!(4.0)))));
        assert_eq!(expression("1 ^ 2 * 3"), Ok(("", multiply!(power!(number!(1.0), number!(2.0)), number!(3.0))))); // 1 ^ 2 * 3 = 1 ^ (2 * 3)
        assert_eq!(expression("1 * 2 ^ 3"), Ok(("", multiply!(number!(1.0), power!(number!(2.0), number!(3.0))))));
    }

    #[test]
    fn complex_stress() {
        assert_eq!(expression("A * (E_0/rho_0)^(1/5) * t^(2/5)"), Ok(("", multiply!(multiply!(variable!("A"), power!(multiply!(variable!("E_0"), invert!(variable!("rho_0"))), multiply!(number!(1.0), invert!(number!(5.0))))), power!(variable!("t"), multiply!(number!(2.0), invert!(number!(5.0))))))));
    }
}