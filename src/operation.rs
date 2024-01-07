use crate::expression::Expression;

pub struct ApplyOperation<'a>{
    pub operation: Operation,
    pub expression: &'a Expression
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operation{
    Identity,
    Add,
    SubtractSelf,
    SubtractOther,
    Multiply,
    DivideNumerator,
    DivideDenominator,
    Sqrt,
    PowerBase,
    PowerExponent,
    LogBase,
    LogPower,
    Sin,
    ArcSin,
    Cos,
    ArcCos,
    Tan,
    ArcTan,
    Ln,
    Abs,
    Negate
}