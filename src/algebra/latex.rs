use crate::algebra::expression::{AlgebraExpr, AlgebraOp};

pub fn to_latex(expr: &AlgebraExpr) -> String {
    render_expr(expr, 0)
}

fn render_expr(expr: &AlgebraExpr, parent_precedence: u8) -> String {
    match expr {
        AlgebraExpr::Number(value) => format_number(*value),

        AlgebraExpr::Variable(name) => format_variable(name),

        AlgebraExpr::Binary { left, op, right } => match op {
            AlgebraOp::Add => render_binary(left, "+", right, 1, parent_precedence),
            AlgebraOp::Subtract => render_binary(left, "-", right, 1, parent_precedence),
            AlgebraOp::Multiply => render_multiply(left, right, parent_precedence),
            AlgebraOp::Divide => {
                format!(
                    "\\frac{{{}}}{{{}}}",
                    render_expr(left, 0),
                    render_expr(right, 0)
                )
            }
            AlgebraOp::Power => render_power(left, right),
        },
    }
}

fn render_binary(
    left: &AlgebraExpr,
    operator: &str,
    right: &AlgebraExpr,
    precedence: u8,
    parent_precedence: u8,
) -> String {
    let output = format!(
        "{} {} {}",
        render_expr(left, precedence),
        operator,
        render_expr(right, precedence)
    );

    wrap_if_needed(output, precedence, parent_precedence)
}

fn render_multiply(left: &AlgebraExpr, right: &AlgebraExpr, parent_precedence: u8) -> String {
    let precedence = 2;

    let output = match (left, right) {
        // 2 * x -> 2x
        (AlgebraExpr::Number(value), AlgebraExpr::Variable(name)) => {
            format!("{}{}", format_number(*value), format_variable(name))
        }

        // x * 2 -> 2x
        (AlgebraExpr::Variable(name), AlgebraExpr::Number(value)) => {
            format!("{}{}", format_number(*value), format_variable(name))
        }

        // 2 * (x + 1) -> 2\left(x + 1\right)
        (AlgebraExpr::Number(value), AlgebraExpr::Binary { op, .. }) if is_additive_op(op) => {
            format!("{}{}", format_number(*value), render_grouped(right))
        }

        // (x + 1) * 2 -> 2\left(x + 1\right)
        (AlgebraExpr::Binary { op, .. }, AlgebraExpr::Number(value)) if is_additive_op(op) => {
            format!("{}{}", format_number(*value), render_grouped(left))
        }

        // x * y -> x \cdot y
        _ => {
            format!(
                "{} \\cdot {}",
                render_expr(left, precedence),
                render_expr(right, precedence)
            )
        }
    };

    wrap_if_needed(output, precedence, parent_precedence)
}

fn render_power(left: &AlgebraExpr, right: &AlgebraExpr) -> String {
    let base = match left {
        AlgebraExpr::Binary { op, .. } if is_additive_op(op) => render_grouped(left),
        _ => render_expr(left, 3),
    };

    let exponent = render_expr(right, 0);

    format!("{}^{{{}}}", base, exponent)
}

fn render_grouped(expr: &AlgebraExpr) -> String {
    format!("\\left({}\\right)", render_expr(expr, 0))
}

fn wrap_if_needed(output: String, precedence: u8, parent_precedence: u8) -> String {
    if precedence < parent_precedence {
        format!("\\left({}\\right)", output)
    } else {
        output
    }
}

fn is_additive_op(op: &AlgebraOp) -> bool {
    matches!(op, AlgebraOp::Add | AlgebraOp::Subtract)
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        value.to_string()
    }
}

fn format_variable(name: &str) -> String {
    match name {
        "alpha" => "\\alpha".to_string(),
        "beta" => "\\beta".to_string(),
        "gamma" => "\\gamma".to_string(),
        "delta" => "\\delta".to_string(),
        "epsilon" => "\\epsilon".to_string(),
        "theta" => "\\theta".to_string(),
        "lambda" => "\\lambda".to_string(),
        "mu" => "\\mu".to_string(),
        "pi" => "\\pi".to_string(),
        "rho" => "\\rho".to_string(),
        "sigma" => "\\sigma".to_string(),
        "phi" => "\\phi".to_string(),
        "omega" => "\\omega".to_string(),

        _ => name.replace('_', "\\_"),
    }
}
