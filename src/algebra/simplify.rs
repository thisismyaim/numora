use crate::algebra::{AlgebraExpr, AlgebraOp};

pub fn simplify_expression(expr: AlgebraExpr) -> AlgebraExpr {
    match expr {
        AlgebraExpr::Binary { op, left, right } => {
            let left = simplify_expression(*left);
            let right = simplify_expression(*right);

            simplify_binary(op, left, right)
        }
        other => other,
    }
}

fn simplify_binary(op: AlgebraOp, left: AlgebraExpr, right: AlgebraExpr) -> AlgebraExpr {
    match op {
        AlgebraOp::Add => simplify_add(left, right),
        AlgebraOp::Subtract => simplify_subtract(left, right),
        AlgebraOp::Multiply => simplify_multiply(left, right),
        AlgebraOp::Divide => simplify_divide(left, right),
        AlgebraOp::Power => simplify_power(left, right),
    }
}

fn simplify_add(left: AlgebraExpr, right: AlgebraExpr) -> AlgebraExpr {
    if left.is_zero() {
        return right;
    }

    if right.is_zero() {
        return left;
    }

    if let (AlgebraExpr::Number(a), AlgebraExpr::Number(b)) = (&left, &right) {
        return AlgebraExpr::Number(a + b);
    }

    AlgebraExpr::add(left, right)
}

fn simplify_subtract(left: AlgebraExpr, right: AlgebraExpr) -> AlgebraExpr {
    if right.is_zero() {
        return left;
    }

    if let (AlgebraExpr::Number(a), AlgebraExpr::Number(b)) = (&left, &right) {
        return AlgebraExpr::Number(a - b);
    }

    AlgebraExpr::subtract(left, right)
}

fn simplify_multiply(left: AlgebraExpr, right: AlgebraExpr) -> AlgebraExpr {
    if left.is_zero() || right.is_zero() {
        return AlgebraExpr::Number(0.0);
    }

    if left.is_one() {
        return right;
    }

    if right.is_one() {
        return left;
    }

    if let (AlgebraExpr::Number(a), AlgebraExpr::Number(b)) = (&left, &right) {
        return AlgebraExpr::Number(a * b);
    }

    AlgebraExpr::multiply(left, right)
}

fn simplify_divide(left: AlgebraExpr, right: AlgebraExpr) -> AlgebraExpr {
    if left.is_zero() {
        return AlgebraExpr::Number(0.0);
    }

    if right.is_one() {
        return left;
    }

    if let (AlgebraExpr::Number(a), AlgebraExpr::Number(b)) = (&left, &right) {
        return AlgebraExpr::Number(a / b);
    }

    AlgebraExpr::divide(left, right)
}

fn simplify_power(left: AlgebraExpr, right: AlgebraExpr) -> AlgebraExpr {
    if right.is_zero() {
        return AlgebraExpr::Number(1.0);
    }

    if right.is_one() {
        return left;
    }

    if let (AlgebraExpr::Number(a), AlgebraExpr::Number(b)) = (&left, &right) {
        return AlgebraExpr::Number(a.powf(*b));
    }

    AlgebraExpr::power(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplifies_add_zero() {
        let expr = AlgebraExpr::add(AlgebraExpr::variable("x"), AlgebraExpr::number(0.0));

        let simplified = simplify_expression(expr);

        assert_eq!(simplified.to_string(), "x");
    }

    #[test]
    fn simplifies_multiply_one() {
        let expr = AlgebraExpr::multiply(AlgebraExpr::number(1.0), AlgebraExpr::variable("x"));

        let simplified = simplify_expression(expr);

        assert_eq!(simplified.to_string(), "x");
    }

    #[test]
    fn simplifies_multiply_zero() {
        let expr = AlgebraExpr::multiply(AlgebraExpr::number(0.0), AlgebraExpr::variable("x"));

        let simplified = simplify_expression(expr);

        assert_eq!(simplified.to_string(), "0");
    }

    #[test]
    fn simplifies_numeric_addition() {
        let expr = AlgebraExpr::add(AlgebraExpr::number(2.0), AlgebraExpr::number(3.0));

        let simplified = simplify_expression(expr);

        assert_eq!(simplified.to_string(), "5");
    }

    #[test]
    fn simplifies_nested_expression() {
        let expr = AlgebraExpr::multiply(
            AlgebraExpr::add(AlgebraExpr::number(2.0), AlgebraExpr::number(3.0)),
            AlgebraExpr::number(1.0),
        );

        let simplified = simplify_expression(expr);

        assert_eq!(simplified.to_string(), "5");
    }
}
