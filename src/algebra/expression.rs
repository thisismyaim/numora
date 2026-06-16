use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AlgebraExpr {
    Number(f64),
    Variable(String),
    Binary {
        op: AlgebraOp,
        left: Box<AlgebraExpr>,
        right: Box<AlgebraExpr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgebraOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

impl AlgebraExpr {
    pub fn number(value: f64) -> Self {
        Self::Number(value)
    }

    pub fn variable(name: impl Into<String>) -> Self {
        Self::Variable(name.into())
    }

    pub fn add(left: AlgebraExpr, right: AlgebraExpr) -> Self {
        Self::Binary {
            op: AlgebraOp::Add,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn subtract(left: AlgebraExpr, right: AlgebraExpr) -> Self {
        Self::Binary {
            op: AlgebraOp::Subtract,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn multiply(left: AlgebraExpr, right: AlgebraExpr) -> Self {
        Self::Binary {
            op: AlgebraOp::Multiply,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn divide(left: AlgebraExpr, right: AlgebraExpr) -> Self {
        Self::Binary {
            op: AlgebraOp::Divide,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn power(left: AlgebraExpr, right: AlgebraExpr) -> Self {
        Self::Binary {
            op: AlgebraOp::Power,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn is_zero(&self) -> bool {
        matches!(self, AlgebraExpr::Number(value) if *value == 0.0)
    }

    pub fn is_one(&self) -> bool {
        matches!(self, AlgebraExpr::Number(value) if *value == 1.0)
    }
}

impl fmt::Display for AlgebraExpr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlgebraExpr::Number(value) => {
                if value.fract() == 0.0 {
                    write!(formatter, "{}", *value as i64)
                } else {
                    write!(formatter, "{}", value)
                }
            }
            AlgebraExpr::Variable(name) => write!(formatter, "{}", name),
            AlgebraExpr::Binary { op, left, right } => {
                write!(formatter, "({} {} {})", left, op, right)
            }
        }
    }
}

impl fmt::Display for AlgebraOp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlgebraOp::Add => write!(formatter, "+"),
            AlgebraOp::Subtract => write!(formatter, "-"),
            AlgebraOp::Multiply => write!(formatter, "*"),
            AlgebraOp::Divide => write!(formatter, "/"),
            AlgebraOp::Power => write!(formatter, "^"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_number_without_decimal_when_whole() {
        let expr = AlgebraExpr::number(5.0);

        assert_eq!(expr.to_string(), "5");
    }

    #[test]
    fn displays_variable() {
        let expr = AlgebraExpr::variable("x");

        assert_eq!(expr.to_string(), "x");
    }

    #[test]
    fn displays_binary_expression() {
        let expr = AlgebraExpr::add(AlgebraExpr::variable("x"), AlgebraExpr::number(2.0));

        assert_eq!(expr.to_string(), "(x + 2)");
    }
}
