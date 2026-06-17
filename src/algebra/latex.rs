use crate::algebra::{AlgebraExpr, AlgebraOp};

pub fn to_latex(expr: &AlgebraExpr) -> String {
    render_expr(expr, 0)
}

fn render_expr(expr: &AlgebraExpr, parent_precedence: u8) -> String {
    match expr {
        AlgebraExpr::Number(value) => format_number(*value),
        AlgebraExpr::Variable(name) => escape_latex_identifier(name),
        AlgebraExpr::Binary { op, left, right } => {
            let precedence = precedence(*op);

            let rendered = match op {
                AlgebraOp::Add => format!(
                    "{} + {}",
                    render_expr(left, precedence),
                    render_expr(right, precedence)
                ),
                AlgebraOp::Subtract => format!(
                    "{} - {}",
                    render_expr(left, precedence),
                    render_expr(right, precedence + 1)
                ),
                AlgebraOp::Multiply => render_multiply(left, right, precedence),
                AlgebraOp::Divide => format!(
                    "\\frac{{{}}}{{{}}}",
                    render_expr(left, 0),
                    render_expr(right, 0)
                ),
                AlgebraOp::Power => {
                    format!("{}^{{{}}}", render_power_base(left), render_expr(right, 0))
                }
            };

            if precedence < parent_precedence {
                format!("\\left({}\\right)", rendered)
            } else {
                rendered
            }
        }
    }
}

fn render_multiply(left: &AlgebraExpr, right: &AlgebraExpr, precedence: u8) -> String {
    match (left, right) {
        // 2 * x -> 2x
        (AlgebraExpr::Number(_), AlgebraExpr::Variable(_)) => {
            format!(
                "{}{}",
                render_expr(left, precedence),
                render_expr(right, precedence)
            )
        }

        // x * 2 -> 2x for cleaner math notation
        (AlgebraExpr::Variable(_), AlgebraExpr::Number(_)) => {
            format!(
                "{}{}",
                render_expr(right, precedence),
                render_expr(left, precedence)
            )
        }

        // 2 * (x + 1) -> 2\left(x + 1\right)
        (AlgebraExpr::Number(_), AlgebraExpr::Binary { op, .. })
            if *op == AlgebraOp::Add || *op == AlgebraOp::Subtract =>
        {
            format!(
                "{}\\left({}\\right)",
                render_expr(left, precedence),
                render_expr(right, 0)
            )
        }

        // (x + 1) * 2 -> 2\left(x + 1\right)
        (AlgebraExpr::Binary { op, .. }, AlgebraExpr::Number(_))
            if *op == AlgebraOp::Add || *op == AlgebraOp::Subtract =>
        {
            format!(
                "{}\\left({}\\right)",
                render_expr(right, precedence),
                render_expr(left, 0)
            )
        }

        _ => format!(
            "{} \\cdot {}",
            render_expr(left, precedence),
            render_expr(right, precedence)
        ),
    }
}

fn render_power_base(expr: &AlgebraExpr) -> String {
    match expr {
        AlgebraExpr::Number(_) | AlgebraExpr::Variable(_) => render_expr(expr, 0),
        AlgebraExpr::Binary { .. } => format!("\\left({}\\right)", render_expr(expr, 0)),
    }
}

fn precedence(op: AlgebraOp) -> u8 {
    match op {
        AlgebraOp::Add | AlgebraOp::Subtract => 1,
        AlgebraOp::Multiply | AlgebraOp::Divide => 2,
        AlgebraOp::Power => 3,
    }
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{}", value)
    }
}

fn escape_latex_identifier(name: &str) -> String {
    match name {
        "alpha" => "\\alpha".to_string(),
        "beta" => "\\beta".to_string(),
        "gamma" => "\\gamma".to_string(),
        "delta" => "\\delta".to_string(),
        "theta" => "\\theta".to_string(),
        "lambda" => "\\lambda".to_string(),
        "mu" => "\\mu".to_string(),
        "pi" => "\\pi".to_string(),
        "sigma" => "\\sigma".to_string(),
        "omega" => "\\omega".to_string(),
        _ => name.replace('_', "\\_"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::parse_algebra_expression;

    fn latex(source: &str) -> String {
        let expr = parse_algebra_expression(source).unwrap();
        to_latex(&expr)
    }

    #[test]
    fn renders_number() {
        assert_eq!(latex("5"), "5");
    }

    #[test]
    fn renders_variable() {
        assert_eq!(latex("x"), "x");
    }

    #[test]
    fn renders_addition() {
        assert_eq!(latex("x + y"), "x + y");
    }

    #[test]
    fn renders_subtraction() {
        assert_eq!(latex("x - y"), "x - y");
    }

    #[test]
    fn renders_numeric_variable_multiplication() {
        assert_eq!(latex("2 * x"), "2x");
    }

    #[test]
    fn renders_variable_numeric_multiplication_cleanly() {
        assert_eq!(latex("x * 2"), "2x");
    }

    #[test]
    fn renders_general_multiplication_with_dot() {
        assert_eq!(latex("x * y"), "x \\cdot y");
    }

    #[test]
    fn renders_fraction() {
        assert_eq!(latex("x / y"), "\\frac{x}{y}");
    }

    #[test]
    fn renders_power() {
        assert_eq!(latex("x ^ 2"), "x^{2}");
    }

    #[test]
    fn renders_power_with_grouped_base() {
        assert_eq!(latex("(x + 1) ^ 2"), "\\left(x + 1\\right)^{2}");
    }

    #[test]
    fn renders_precedence() {
        assert_eq!(latex("x + 2 * y"), "x + 2y");
    }

    #[test]
    fn renders_parentheses_for_multiplication_group() {
        assert_eq!(latex("2 * (x + 1)"), "2\\left(x + 1\\right)");
    }

    #[test]
    fn renders_greek_symbol_names() {
        assert_eq!(latex("theta + lambda"), "\\theta + \\lambda");
    }

    #[test]
    fn escapes_underscores() {
        assert_eq!(latex("x_1 + y_2"), "x\\_1 + y\\_2");
    }
}
