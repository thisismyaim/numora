use crate::algebra::expression::{AlgebraExpr, AlgebraOp};
use crate::algebra::simplify_expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgebraExplanationStep {
    pub before: String,
    pub after: String,
    pub rule: String,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgebraExplanation {
    pub original: String,
    pub simplified: String,
    pub steps: Vec<AlgebraExplanationStep>,
}

pub fn explain_simplification(expr: AlgebraExpr) -> AlgebraExplanation {
    let simplified = simplify_expression(expr.clone());

    let mut steps = Vec::new();
    collect_steps(&expr, &simplified, &mut steps);

    if steps.is_empty() && expr != simplified {
        steps.push(AlgebraExplanationStep {
            before: expr.to_string(),
            after: simplified.to_string(),
            rule: "Simplification".to_string(),
            explanation: "The expression was simplified to an equivalent form.".to_string(),
        });
    }

    AlgebraExplanation {
        original: expr.to_string(),
        simplified: simplified.to_string(),
        steps,
    }
}

fn collect_steps(
    before: &AlgebraExpr,
    after: &AlgebraExpr,
    steps: &mut Vec<AlgebraExplanationStep>,
) {
    if before == after {
        return;
    }

    if let Some(step) = explain_direct_rule(before, after) {
        steps.push(step);
        return;
    }

    match before {
        AlgebraExpr::Binary { left, op: _, right } => {
            let simplified_left = simplify_expression((**left).clone());
            let simplified_right = simplify_expression((**right).clone());

            collect_steps(left, &simplified_left, steps);
            collect_steps(right, &simplified_right, steps);
        }

        AlgebraExpr::Number(_) | AlgebraExpr::Variable(_) => {}
    }

    if before != after {
        steps.push(AlgebraExplanationStep {
            before: before.to_string(),
            after: after.to_string(),
            rule: "Final simplification".to_string(),
            explanation: "After applying algebra rules, the expression becomes simpler."
                .to_string(),
        });
    }
}

fn explain_direct_rule(
    before: &AlgebraExpr,
    after: &AlgebraExpr,
) -> Option<AlgebraExplanationStep> {
    match before {
        AlgebraExpr::Binary { left, op, right } => match op {
            AlgebraOp::Add => explain_addition(left, right, before, after),
            AlgebraOp::Subtract => explain_subtraction(left, right, before, after),
            AlgebraOp::Multiply => explain_multiplication(left, right, before, after),
            AlgebraOp::Divide => explain_division(right, before, after),
            AlgebraOp::Power => explain_power(right, before, after),
        },

        AlgebraExpr::Number(_) | AlgebraExpr::Variable(_) => None,
    }
}

fn explain_addition(
    left: &AlgebraExpr,
    right: &AlgebraExpr,
    before: &AlgebraExpr,
    after: &AlgebraExpr,
) -> Option<AlgebraExplanationStep> {
    if is_zero(left) || is_zero(right) {
        return Some(make_step(
            before,
            after,
            "Additive identity",
            "Adding zero does not change the value of an expression.",
        ));
    }

    if left == right {
        return Some(make_step(
            before,
            after,
            "Combine like terms",
            "Adding the same variable to itself gives two copies of that variable.",
        ));
    }

    if are_like_variable_terms(left, right) {
        return Some(make_step(
            before,
            after,
            "Combine like terms",
            "Terms with the same variable part can be combined by adding their coefficients.",
        ));
    }

    if is_number(left) && is_number(right) {
        return Some(make_step(
            before,
            after,
            "Evaluate constants",
            "Numeric constants can be calculated directly.",
        ));
    }

    None
}

fn explain_subtraction(
    left: &AlgebraExpr,
    right: &AlgebraExpr,
    before: &AlgebraExpr,
    after: &AlgebraExpr,
) -> Option<AlgebraExplanationStep> {
    if is_zero(right) {
        return Some(make_step(
            before,
            after,
            "Subtractive identity",
            "Subtracting zero does not change the value of an expression.",
        ));
    }

    if left == right {
        return Some(make_step(
            before,
            after,
            "Self subtraction",
            "Any expression minus itself equals zero.",
        ));
    }

    if is_number(left) && is_number(right) {
        return Some(make_step(
            before,
            after,
            "Evaluate constants",
            "Numeric constants can be calculated directly.",
        ));
    }

    None
}

fn explain_multiplication(
    left: &AlgebraExpr,
    right: &AlgebraExpr,
    before: &AlgebraExpr,
    after: &AlgebraExpr,
) -> Option<AlgebraExplanationStep> {
    if is_zero(left) || is_zero(right) {
        return Some(make_step(
            before,
            after,
            "Zero product",
            "Any expression multiplied by zero equals zero.",
        ));
    }

    if is_one(left) || is_one(right) {
        return Some(make_step(
            before,
            after,
            "Multiplicative identity",
            "Multiplying by one does not change the value of an expression.",
        ));
    }

    if is_number(left) && is_number(right) {
        return Some(make_step(
            before,
            after,
            "Evaluate constants",
            "Numeric constants can be calculated directly.",
        ));
    }

    None
}

fn explain_division(
    right: &AlgebraExpr,
    before: &AlgebraExpr,
    after: &AlgebraExpr,
) -> Option<AlgebraExplanationStep> {
    if is_one(right) {
        return Some(make_step(
            before,
            after,
            "Division identity",
            "Dividing by one does not change the value of an expression.",
        ));
    }

    None
}

fn explain_power(
    right: &AlgebraExpr,
    before: &AlgebraExpr,
    after: &AlgebraExpr,
) -> Option<AlgebraExplanationStep> {
    if is_one(right) {
        return Some(make_step(
            before,
            after,
            "Power of one rule",
            "Any expression raised to the power of one stays the same.",
        ));
    }

    if is_zero(right) {
        return Some(make_step(
            before,
            after,
            "Power of zero rule",
            "Any non-zero expression raised to the power of zero equals one.",
        ));
    }

    None
}

fn make_step(
    before: &AlgebraExpr,
    after: &AlgebraExpr,
    rule: &str,
    explanation: &str,
) -> AlgebraExplanationStep {
    AlgebraExplanationStep {
        before: before.to_string(),
        after: after.to_string(),
        rule: rule.to_string(),
        explanation: explanation.to_string(),
    }
}

fn is_number(expr: &AlgebraExpr) -> bool {
    matches!(expr, AlgebraExpr::Number(_))
}

fn is_zero(expr: &AlgebraExpr) -> bool {
    matches!(expr, AlgebraExpr::Number(value) if *value == 0.0)
}

fn is_one(expr: &AlgebraExpr) -> bool {
    matches!(expr, AlgebraExpr::Number(value) if *value == 1.0)
}

fn are_like_variable_terms(left: &AlgebraExpr, right: &AlgebraExpr) -> bool {
    extract_variable_name(left).is_some()
        && extract_variable_name(left) == extract_variable_name(right)
}

fn extract_variable_name(expr: &AlgebraExpr) -> Option<String> {
    match expr {
        AlgebraExpr::Variable(name) => Some(name.clone()),

        AlgebraExpr::Binary { left, op, right } if matches!(op, AlgebraOp::Multiply) => {
            match (left.as_ref(), right.as_ref()) {
                (AlgebraExpr::Number(_), AlgebraExpr::Variable(name)) => Some(name.clone()),
                (AlgebraExpr::Variable(name), AlgebraExpr::Number(_)) => Some(name.clone()),
                _ => None,
            }
        }

        _ => None,
    }
}
