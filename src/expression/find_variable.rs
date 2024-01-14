// use crate::expression::Expression;
// use crate::operation::{ApplyOperation, Operation};


// impl Expression {
    // pub fn contains_variable(&self, variable: &str) -> bool {
    //     use Expression::*;
    //     match self {
    //         Constant(_) |
    //         Number(_)
    //             => false,
    //         Variable(name)
    //             => name == variable,
    //         Add(left, right) |
    //         Subtract(left, right) |
    //         Multiply(left, right) |
    //         Divide(left, right) |
    //         Power(left, right) |
    //         Log(left, right)
    //             => left.contains_variable(variable) || right.contains_variable(variable),
    //         Sqrt(inner) |
    //         Sin(inner) |
    //         ArcSin(inner) |
    //         Cos(inner) |
    //         ArcCos(inner) |
    //         Tan(inner) |
    //         ArcTan(inner) |
    //         Ln(inner) |
    //         Abs(inner) |
    //         Negate(inner)
    //             => inner.contains_variable(variable),
    //     }
    // }

    // pub fn operations_on_variable(&self, variable: &str) -> Option<OperationBranch> {
    //     if let Some(children) = self.children(){
    //         match children{
    //             Children::One(child) => {
    //                 if child.expression.contains_variable(variable){
    //                     return Some(OperationBranch::One(OperationNode{
    //                         operation: child,
    //                         child: Some(Box::new(child.expression.operations_on_variable(variable)?)),
    //                     }))
    //                 }
    //             }
    //             Children::Two(left, right) => {}
    //         }
    //     }
    //     None
    // }

    // pub fn children(&self) -> Option<Children>{
    //     match self{
    //         Expression::Constant(_) |
    //         Expression::Number(_) |
    //         Expression::Variable(_) => None,
    //         Expression::Add(left, right) => Some(Children::Two(
    //             Child{
    //                 expression: left,
    //                 operation: Operation::Add,
    //             },
    //             Child{
    //                 expression: right,
    //                 operation: Operation::Add,
    //             },
    //         )),
    //         Expression::Subtract(left, right) => Some(Children::Two(
    //             Child{
    //                 expression: left,
    //                 operation: Operation::SubtractOther,
    //             },
    //             Child{
    //                 expression: right,
    //                 operation: Operation::SubtractSelf,
    //             },
    //         )),
    //         Expression::Multiply(left, right) => Some(Children::Two(
    //                 Child{
    //                     expression: left,
    //                     operation: Operation::Multiply,
    //                 },
    //                 Child{
    //                     expression: right,
    //                     operation: Operation::Multiply,
    //                 },
    //             )),
    //         Expression::Divide(left, right) => Some(Children::Two(
    //                 Child{
    //                     expression: left,
    //                     operation: Operation::DivideNumerator,
    //                 },
    //                 Child{
    //                     expression: right,
    //                     operation: Operation::DivideDenominator,
    //                 },
    //             )),
    //         Expression::Power(left, right) => Some(Children::Two(
    //                 Child{
    //                     expression: left,
    //                     operation: Operation::PowerBase,
    //                 },
    //                 Child{
    //                     expression: right,
    //                     operation: Operation::PowerExponent,
    //                 },
    //             )),
    //         Expression::Log(left, right) => Some(Children::Two(
    //                 Child{
    //                     expression: left,
    //                     operation: Operation::LogBase,
    //                 },
    //                 Child{
    //                     expression: right,
    //                     operation: Operation::LogPower,
    //                 },
    //             )),
    //         Expression::Sqrt(inner) => Some(Children::One(
    //                 Child{
    //                     expression: inner,
    //                     operation: Operation::Sqrt,
    //                 },
    //             )),
    //         Expression::Sin(inner) => Some(Children::One(
    //                 Child{
    //                     expression: inner,
    //                     operation: Operation::Sin,
    //                 },
    //             )),
    //         Expression::ArcSin(inner) => Some(Children::One(
    //                 Child{
    //                     expression: inner,
    //                     operation: Operation::ArcSin,
    //                 },
    //             )),
    //         Expression::Cos(inner) => Some(Children::One(
    //                 Child{
    //                     expression: inner,
    //                     operation: Operation::Cos,
    //                 },
    //             )),
    //         Expression::ArcCos(inner) => Some(Children::One(
    //                 Child{
    //                     expression: inner,
    //                     operation: Operation::ArcCos,
    //                 },
    //             )),
    //         Expression::Tan(inner) => Some(Children::One(
    //                 Child{
    //                     expression: inner,
    //                     operation: Operation::Tan,
    //                 },
    //             )),
    //         Expression::ArcTan(inner) => Some(Children::One(
    //                 Child{
    //                     expression: inner,
    //                     operation: Operation::ArcTan,
    //                 },
    //             )),
    //         Expression::Ln(inner) => Some(Children::One(
    //                 Child{
    //                     expression: inner,
    //                     operation: Operation::Ln,
    //                 },
    //             )),
    //         Expression::Abs(inner) => Some(Children::One(
    //                 Child{
    //                     expression: inner,
    //                     operation: Operation::Abs,
    //                 },
    //             )),
    //         Expression::Negate(inner) => Some(Children::One(
    //                 Child{
    //                     expression: inner,
    //                     operation: Operation::Negate,
    //                 },
    //             )),
    //     }
    // }
// }
