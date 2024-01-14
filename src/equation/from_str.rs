use nom::bytes::complete::tag;
use nom::error::Error;
use nom::IResult;
use nom::sequence::tuple;
use crate::equation::Equation;
use crate::expression::from_str::{expression, ws};

impl Equation{
    pub fn from_str(input: &str) -> Result<Equation, nom::Err<Error<&str>>> {
        equation(input).map(|(_, equation)| equation)
    }
}

fn equation(input: &str) -> IResult<&str, Equation> {
    let (input, (left, _, right)) = tuple((
        expression,
        ws(tag("=")),
        expression
    ))(input)?;

    return Ok((input, Equation{left, right}));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::equation::Equation;
    use crate::expression::Expression;

    #[test]
    fn test_equation() {
        let equation = Equation::from_str("x^2 + 2*x + 1 = 0").unwrap();
        assert_eq!(equation.left, Expression::from_str("x^2 + 2*x + 1").unwrap());
        assert_eq!(equation.right, Expression::from_str("0").unwrap());
    }
}