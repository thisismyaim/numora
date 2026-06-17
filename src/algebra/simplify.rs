use crate::algebra::{AlgebraExpr, AlgebraOp};

pub fn simplify_expression(expr: AlgebraExpr) -> AlgebraExpr {
    match expr {
        AlgebraExpr::Binary { op, left, right } => {
            let left = simplify_expression(*left);
            let right = simplify_expression(*right);

            let simplified = simplify_binary(op, left, right);

            // Second pass handles cases created by the first pass.
            match simplified {
                AlgebraExpr::Binary { op, left, right } => simplify_binary(op, *left, *right),
                other => other,
            }
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

    if let Some(value) = numeric_binary(&left, &right, |a, b| a + b) {
        return AlgebraExpr::Number(value);
    }

    // x + x -> 2 * x
    if left == right {
        return AlgebraExpr::multiply(AlgebraExpr::number(2.0), left);
    }

    // (a * x) + (b * x) -> (a + b) * x
    if let Some((left_coeff, left_term)) = split_numeric_coefficient(&left) {
        if let Some((right_coeff, right_term)) = split_numeric_coefficient(&right) {
            if left_term == right_term {
                return simplify_multiply(AlgebraExpr::number(left_coeff + right_coeff), left_term);
            }
        }
    }

    // x + (b * x) -> (1 + b) * x
    if let Some((right_coeff, right_term)) = split_numeric_coefficient(&right) {
        if left == right_term {
            return simplify_multiply(AlgebraExpr::number(1.0 + right_coeff), left);
        }
    }

    // (a * x) + x -> (a + 1) * x
    if let Some((left_coeff, left_term)) = split_numeric_coefficient(&left) {
        if left_term == right {
            return simplify_multiply(AlgebraExpr::number(left_coeff + 1.0), right);
        }
    }

    // (x + a) + b -> x + (a + b)
    if let AlgebraExpr::Binary {
        op: AlgebraOp::Add,
        left: nested_left,
        right: nested_right,
    } = &left
    {
        if let (AlgebraExpr::Number(a), AlgebraExpr::Number(b)) = (&**nested_right, &right) {
            return simplify_add((**nested_left).clone(), AlgebraExpr::number(a + b));
        }
    }

    // a + (x + b) -> x + (a + b)
    if let AlgebraExpr::Binary {
        op: AlgebraOp::Add,
        left: nested_left,
        right: nested_right,
    } = &right
    {
        if let (AlgebraExpr::Number(a), AlgebraExpr::Number(b)) = (&left, &**nested_right) {
            return simplify_add((**nested_left).clone(), AlgebraExpr::number(a + b));
        }
    }

    AlgebraExpr::add(left, right)
}

fn simplify_subtract(left: AlgebraExpr, right: AlgebraExpr) -> AlgebraExpr {
    if right.is_zero() {
        return left;
    }

    if left == right {
        return AlgebraExpr::number(0.0);
    }

    if let Some(value) = numeric_binary(&left, &right, |a, b| a - b) {
        return AlgebraExpr::Number(value);
    }

    // (a * x) - (b * x) -> (a - b) * x
    if let Some((left_coeff, left_term)) = split_numeric_coefficient(&left) {
        if let Some((right_coeff, right_term)) = split_numeric_coefficient(&right) {
            if left_term == right_term {
                return simplify_multiply(AlgebraExpr::number(left_coeff - right_coeff), left_term);
            }
        }
    }

    // x - (b * x) -> (1 - b) * x
    if let Some((right_coeff, right_term)) = split_numeric_coefficient(&right) {
        if left == right_term {
            return simplify_multiply(AlgebraExpr::number(1.0 - right_coeff), left);
        }
    }

    // (a * x) - x -> (a - 1) * x
    if let Some((left_coeff, left_term)) = split_numeric_coefficient(&left) {
        if left_term == right {
            return simplify_multiply(AlgebraExpr::number(left_coeff - 1.0), right);
        }
    }

    AlgebraExpr::subtract(left, right)
}

fn simplify_multiply(left: AlgebraExpr, right: AlgebraExpr) -> AlgebraExpr {
    if left.is_zero() || right.is_zero() {
        return AlgebraExpr::number(0.0);
    }

    if left.is_one() {
        return right;
    }

    if right.is_one() {
        return left;
    }

    if let Some(value) = numeric_binary(&left, &right, |a, b| a * b) {
        return AlgebraExpr::Number(value);
    }

    // x * x -> x ^ 2
    if left == right {
        return AlgebraExpr::power(left, AlgebraExpr::number(2.0));
    }

    // (a * x) * b -> (a * b) * x
    if let Some((left_coeff, left_term)) = split_numeric_coefficient(&left) {
        if let AlgebraExpr::Number(right_number) = right {
            return simplify_multiply(AlgebraExpr::number(left_coeff * right_number), left_term);
        }
    }

    // a * (b * x) -> (a * b) * x
    if let AlgebraExpr::Number(left_number) = left {
        if let Some((right_coeff, right_term)) = split_numeric_coefficient(&right) {
            return simplify_multiply(AlgebraExpr::number(left_number * right_coeff), right_term);
        }

        return AlgebraExpr::multiply(AlgebraExpr::number(left_number), right);
    }

    AlgebraExpr::multiply(left, right)
}

fn simplify_divide(left: AlgebraExpr, right: AlgebraExpr) -> AlgebraExpr {
    if left.is_zero() {
        return AlgebraExpr::number(0.0);
    }

    if right.is_one() {
        return left;
    }

    if left == right {
        return AlgebraExpr::number(1.0);
    }

    if let Some(value) = numeric_binary(&left, &right, |a, b| a / b) {
        return AlgebraExpr::Number(value);
    }

    AlgebraExpr::divide(left, right)
}

fn simplify_power(left: AlgebraExpr, right: AlgebraExpr) -> AlgebraExpr {
    if right.is_zero() {
        return AlgebraExpr::number(1.0);
    }

    if right.is_one() {
        return left;
    }

    if left.is_one() {
        return AlgebraExpr::number(1.0);
    }

    if left.is_zero() {
        return AlgebraExpr::number(0.0);
    }

    if let Some(value) = numeric_binary(&left, &right, |a, b| a.powf(b)) {
        return AlgebraExpr::Number(value);
    }

    AlgebraExpr::power(left, right)
}

fn numeric_binary<F>(left: &AlgebraExpr, right: &AlgebraExpr, operation: F) -> Option<f64>
where
    F: FnOnce(f64, f64) -> f64,
{
    if let (AlgebraExpr::Number(a), AlgebraExpr::Number(b)) = (left, right) {
        Some(operation(*a, *b))
    } else {
        None
    }
}

fn split_numeric_coefficient(expr: &AlgebraExpr) -> Option<(f64, AlgebraExpr)> {
    match expr {
        AlgebraExpr::Binary {
            op: AlgebraOp::Multiply,
            left,
            right,
        } => match (&**left, &**right) {
            (AlgebraExpr::Number(coeff), term) => Some((*coeff, term.clone())),
            (term, AlgebraExpr::Number(coeff)) => Some((*coeff, term.clone())),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::parse_algebra_expression;

    fn simplify_text(source: &str) -> String {
        let expr = parse_algebra_expression(source).unwrap();
        simplify_expression(expr).to_string()
    }

    #[test]
    fn simplifies_add_zero() {
        assert_eq!(simplify_text("x + 0"), "x");
        assert_eq!(simplify_text("0 + x"), "x");
    }

    #[test]
    fn simplifies_subtract_zero() {
        assert_eq!(simplify_text("x - 0"), "x");
    }

    #[test]
    fn simplifies_subtract_same_term() {
        assert_eq!(simplify_text("x - x"), "0");
    }

    #[test]
    fn simplifies_multiply_one() {
        assert_eq!(simplify_text("1 * x"), "x");
        assert_eq!(simplify_text("x * 1"), "x");
    }

    #[test]
    fn simplifies_multiply_zero() {
        assert_eq!(simplify_text("0 * x"), "0");
        assert_eq!(simplify_text("x * 0"), "0");
    }

    #[test]
    fn simplifies_divide_one() {
        assert_eq!(simplify_text("x / 1"), "x");
    }

    #[test]
    fn simplifies_zero_divided_by_term() {
        assert_eq!(simplify_text("0 / x"), "0");
    }

    #[test]
    fn simplifies_term_divided_by_same_term() {
        assert_eq!(simplify_text("x / x"), "1");
    }

    #[test]
    fn simplifies_power_one() {
        assert_eq!(simplify_text("x ^ 1"), "x");
    }

    #[test]
    fn simplifies_power_zero() {
        assert_eq!(simplify_text("x ^ 0"), "1");
    }

    #[test]
    fn simplifies_one_power_term() {
        assert_eq!(simplify_text("1 ^ x"), "1");
    }

    #[test]
    fn simplifies_zero_power_term() {
        assert_eq!(simplify_text("0 ^ x"), "0");
    }

    #[test]
    fn simplifies_numeric_addition() {
        assert_eq!(simplify_text("2 + 3"), "5");
    }

    #[test]
    fn simplifies_numeric_precedence_expression() {
        assert_eq!(simplify_text("2 + 3 * 4"), "14");
    }

    #[test]
    fn simplifies_nested_expression() {
        assert_eq!(simplify_text("(2 + 3) * 1"), "5");
    }

    #[test]
    fn combines_same_variable_addition() {
        assert_eq!(simplify_text("x + x"), "(2 * x)");
    }

    #[test]
    fn combines_numeric_coefficients() {
        assert_eq!(simplify_text("2 * x + 3 * x"), "(5 * x)");
    }

    #[test]
    fn combines_variable_with_coefficient_right() {
        assert_eq!(simplify_text("x + 3 * x"), "(4 * x)");
    }

    #[test]
    fn combines_variable_with_coefficient_left() {
        assert_eq!(simplify_text("3 * x + x"), "(4 * x)");
    }

    #[test]
    fn subtracts_numeric_coefficients() {
        assert_eq!(simplify_text("5 * x - 2 * x"), "(3 * x)");
    }

    #[test]
    fn simplifies_x_times_x_to_power() {
        assert_eq!(simplify_text("x * x"), "(x ^ 2)");
    }

    #[test]
    fn combines_nested_numeric_constants() {
        assert_eq!(simplify_text("x + 2 + 3"), "(x + 5)");
    }
}
