#[macro_export]
macro_rules! var {
    ($expression:expr) => {
        $crate::expression::Expression::Variable(String::from($expression))
    };
}

#[macro_export]
macro_rules! num {
    ($expression:expr) => {
        $crate::expression::Expression::Number($crate::expression::number::Number($expression as f64))
    };
}

#[macro_export]
macro_rules! add {
    ($($expression:expr),+ $(,)?) => {
        $crate::expression::Expression::Add($crate::expression::add::Add( vec![$($expression),+]))
    };
}

#[macro_export]
macro_rules! mul {
    ($($expression:expr),+ $(,)?) => {
        $crate::expression::Expression::Multiply($crate::expression::multiply::Multiply( vec![$($expression),+]))
    };
}

#[macro_export]
macro_rules! div {
    ($expression1:expr, $expression2:expr $(,)?) => {
        Expression::Divide(Box::new($expression1), Box::new($expression2))
    };
}

#[macro_export]
macro_rules! pow {
    ($expression1:expr, $expression2:expr $(,)?) => {
        Expression::Power(Box::new($expression1), Box::new($expression2))
    };
}

#[macro_export]
macro_rules! neg {
    ($expression:expr) => {
        Expression::Negate($crate::expression::Negate(Box::new($expression)))
    };
}

#[macro_export]
macro_rules! inv {
    ($expression:expr) => {
        Expression::Invert(($crate::expression::Invert(Box::new($expression))))
    }
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
macro_rules! asin {
    ($expression:expr) => {
        Expression::ArcSin(Box::new($expression))
    };
}

#[macro_export]
macro_rules! acos {
    ($expression:expr) => {
        Expression::ArcCos(Box::new($expression))
    };
}

#[macro_export]
macro_rules! atan {
    ($expression:expr) => {
        Expression::ArcTan(Box::new($expression))
    };
}
