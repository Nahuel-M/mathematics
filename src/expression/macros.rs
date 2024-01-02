#[macro_export]
macro_rules! variable {
    ($expression:expr) => {
        Expression::Variable(String::from($expression))
    };
}

#[macro_export]
macro_rules! number {
    ($expression:expr) => {
        Expression::Number($expression)
    };
}

#[macro_export]
macro_rules! add {
    ($expression1:expr, $expression2:expr $(,)?) => {
        crate::expression::Expression::Add(Box::new($expression1), Box::new($expression2))
    };
}

#[macro_export]
macro_rules! subtract {
    ($expression1:expr, $expression2:expr $(,)?) => {
        Expression::Subtract(Box::new($expression1), Box::new($expression2))
    };
}

#[macro_export]
macro_rules! multiply {
    ($expression1:expr, $expression2:expr $(,)?) => {
        Expression::Multiply(Box::new($expression1), Box::new($expression2))
    };
}

#[macro_export]
macro_rules! divide {
    ($expression1:expr, $expression2:expr $(,)?) => {
        Expression::Divide(Box::new($expression1), Box::new($expression2))
    };
}

#[macro_export]
macro_rules! power {
    ($expression1:expr, $expression2:expr $(,)?) => {
        Expression::Power(Box::new($expression1), Box::new($expression2))
    };
}

#[macro_export]
macro_rules! negate {
    ($expression:expr) => {
        Expression::Negate(Box::new($expression))
    };
}

#[macro_export]
macro_rules! sqrt {
    ($expression:expr) => {
        Expression::Sqrt(Box::new($expression))
    };
}

#[macro_export]
macro_rules! abs {
    ($expression:expr) => {
        Expression::Abs(Box::new($expression))
    };
}

#[macro_export]
macro_rules! log {
    ($expression1:expr, $expression2:expr $(,)?) => {
        Expression::Log(Box::new($expression1), Box::new($expression2))
    };
}

#[macro_export]
macro_rules! exp {
    ($expression:expr) => {
        Expression::Exp(Box::new($expression))
    };
}

#[macro_export]
macro_rules! ln {
    ($expression:expr) => {
        Expression::Ln(Box::new($expression))
    };
}

#[macro_export]
macro_rules! sin {
    ($expression:expr) => {
        Expression::Sin(Box::new($expression))
    };
}

#[macro_export]
macro_rules! cos {
    ($expression:expr) => {
        Expression::Cos(Box::new($expression))
    };
}

#[macro_export]
macro_rules! tan {
    ($expression:expr) => {
        Expression::Tan(Box::new($expression))
    };
}

#[macro_export]
macro_rules! arcsin {
    ($expression:expr) => {
        Expression::ArcSin(Box::new($expression))
    };
}

#[macro_export]
macro_rules! arccos {
    ($expression:expr) => {
        Expression::ArcCos(Box::new($expression))
    };
}

#[macro_export]
macro_rules! arctan {
    ($expression:expr) => {
        Expression::ArcTan(Box::new($expression))
    };
}
