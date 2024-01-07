use crate::{expression::Expression, number, multiply, negate, power, divide, add, sqrt, cos, sin, tan, arccos, arcsin, arctan};
use crate::expression::add::Add;
use crate::expression::multiply::Multiply;


//
// pub fn simplify_power(a: &Expression, b: &Expression) -> Expression {
//     use Expression::*;
//     match (a.simplify(), b.simplify()) {
//         (Number(a), Number(b)) => number!(a.powf(b)),
//         (_a, Number(b)) if b == 0.0 => number!(1.0),
//         (a, Number(b)) if b == 1.0 => a,
//         (a, b) if a == b => power!(a, number!(2.0)),
//         _ => power!(a.clone(), b.clone()),
//     }
// }
//
// pub fn simplify_sqrt(a: &Expression) -> Expression {
//     use Expression::*;
//     match a.simplify() {
//         Number(a) => number!(a.sqrt()),
//         Power(a, b) if *b == number!(2.0) => *a.clone(),
//         _ => sqrt!(a.clone()),
//     }
// }
//
// pub fn simplify_cos(a: &Expression) -> Expression {
//     use Expression::*;
//     match a.simplify() {
//         Number(a) => number!(a.cos()),
//         _ => cos!(a.clone()),
//     }
// }
//
// pub fn simplify_sin(a: &Expression) -> Expression {
//     use Expression::*;
//     match a.simplify() {
//         Number(a) => number!(a.sin()),
//         _ => sin!(a.clone()),
//     }
// }
//
// pub fn simplify_tan(a: &Expression) -> Expression {
//     use Expression::*;
//     match a.simplify() {
//         Number(a) => number!(a.tan()),
//         Tan(a) => *a.clone(),
//         _ => tan!(a.clone()),
//     }
// }
//
// pub fn simplify_arccos(a: &Expression) -> Expression {
//     use Expression::*;
//     match a.simplify() {
//         Number(a) => number!(a.acos()),
//         Cos(a) => *a.clone(),
//         _ => arccos!(a.clone()),
//     }
// }
//
// pub fn simplify_arcsin(a: &Expression) -> Expression {
//     use Expression::*;
//     match a.simplify() {
//         Number(a) => number!(a.asin()),
//         Sin(a) => *a.clone(),
//         _ => arcsin!(a.clone()),
//     }
// }
//
// pub fn simplify_arctan(a: &Expression) -> Expression {
//     use Expression::*;
//     match a.simplify() {
//         Number(a) => number!(a.atan()),
//         Tan(a) => *a.clone(),
//         _ => arctan!(a.clone()),
//     }
// }
