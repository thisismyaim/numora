use crate::algebra::{simplify_expression, AlgebraExpr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgebraExplanationStep {
    pub before: String,
    pub after: String,
    pub rule: String,
    pub explanation: String,
}

impl AlgebraExplanationStep {
    pub fn new(
        before: impl Into<String>,
        after: impl Into<String>,
        rule: impl Into<String>,
        explanation: impl Into<String>,
    ) -> Self {
        Self {
            before: before.into(),
            after: after.into(),
            rule: rule.into(),
            explanation: explanation.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgebraExplanation {
    pub original: String,
    pub simplified: String,
    pub steps: Vec<AlgebraExplanationStep>,
}

impl AlgebraExplanation {
    pub fn new(
        original: impl Into<String>,
        simplified: impl Into<String>,
        steps: Vec<AlgebraExplanationStep>,
    ) -> Self {
        Self {
            original: original.into(),
            simplified: simplified.into(),
            steps,
        }
    }
}

pub fn explain_simplification(expr: AlgebraExpr) -> AlgebraExplanation {
    let original = expr.to_string();
    let simplified_expr = simplify_expression(expr.clone());
    let simplified = simplified_expr.to_string();

    let mut steps = Vec::new();

    collect_explanation_steps(&expr, &mut steps);

    if original != simplified && steps.is_empty() {
        steps.push(AlgebraExplanationStep::new(
            original.clone(),
            simplified.clone(),
            "Simplification",
            "The expression was simplified using algebraic rules.",
        ));
    }

    AlgebraExplanation::new(original, simplified, steps)
}

fn collect_explanation_steps(expr: &AlgebraExpr, steps: &mut Vec<AlgebraExplanationStep>) {
    match expr {
        AlgebraExpr::Binary { left, right, .. } => {
            collect_explanation_steps(left, steps);
            collect_explanation_steps(right, steps);

            let before = expr.to_string();
            let after = simplify_expression(expr.clone()).to_string();

            if before == after {
                return;
            }

            if let Some(step) = explain_known_rule(expr, &before, &after) {
                steps.push(step);
            }
        }
        _ => {}
    }
}

fn explain_known_rule(
    expr: &AlgebraExpr,
    before: &str,
    after: &str,
) -> Option<AlgebraExplanationStep> {
    match expr {
        AlgebraExpr::Binary { op, left, right } => {
            let op_text = op.to_string();

            match op_text.as_str() {
                "+" => explain_add_rule(left, right, before, after),
                "-" => explain_subtract_rule(left, right, before, after),
                "*" => explain_multiply_rule(left, right, before, after),
                "/" => explain_divide_rule(left, right, before, after),
                "^" => explain_power_rule(left, right, before, after),
                _ => None,
            }
        }
        _ => None,
    }
}

fn explain_add_rule(
    left: &AlgebraExpr,
    right: &AlgebraExpr,
    before: &str,
    after: &str,
) -> Option<AlgebraExplanationStep> {
    if left.is_zero() || right.is_zero() {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Additive identity",
            "Adding zero does not change the value of an expression.",
        ));
    }

    if left == right {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Combine like terms",
            "The same term added to itself becomes two times that term.",
        ));
    }

    if is_numeric(left) && is_numeric(right) {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Numeric addition",
            "Both sides are numbers, so they can be added directly.",
        ));
    }

    if after.contains('*') && before.contains('*') {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Combine like terms",
            "Terms with the same variable part can be combined by adding their coefficients.",
        ));
    }

    None
}

fn explain_subtract_rule(
    left: &AlgebraExpr,
    right: &AlgebraExpr,
    before: &str,
    after: &str,
) -> Option<AlgebraExplanationStep> {
    if right.is_zero() {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Subtractive identity",
            "Subtracting zero does not change the value of an expression.",
        ));
    }

    if left == right {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Self subtraction",
            "Any expression minus itself equals zero.",
        ));
    }

    if is_numeric(left) && is_numeric(right) {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Numeric subtraction",
            "Both sides are numbers, so they can be subtracted directly.",
        ));
    }

    if after.contains('*') && before.contains('*') {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Combine like terms",
            "Terms with the same variable part can be combined by subtracting their coefficients.",
        ));
    }

    None
}

fn explain_multiply_rule(
    left: &AlgebraExpr,
    right: &AlgebraExpr,
    before: &str,
    after: &str,
) -> Option<AlgebraExplanationStep> {
    if left.is_zero() || right.is_zero() {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Zero product property",
            "Anything multiplied by zero becomes zero.",
        ));
    }

    if left.is_one() || right.is_one() {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Multiplicative identity",
            "Multiplying by one does not change the value of an expression.",
        ));
    }

    if left == right {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Repeated multiplication",
            "A term multiplied by itself can be written as that term squared.",
        ));
    }

    if is_numeric(left) && is_numeric(right) {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Numeric multiplication",
            "Both sides are numbers, so they can be multiplied directly.",
        ));
    }

    None
}

fn explain_divide_rule(
    left: &AlgebraExpr,
    right: &AlgebraExpr,
    before: &str,
    after: &str,
) -> Option<AlgebraExplanationStep> {
    if left.is_zero() {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Zero divided by expression",
            "Zero divided by any non-zero expression is zero.",
        ));
    }

    if right.is_one() {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Division identity",
            "Dividing by one does not change the value of an expression.",
        ));
    }

    if left == right {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Self division",
            "Any non-zero expression divided by itself equals one.",
        ));
    }

    if is_numeric(left) && is_numeric(right) {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Numeric division",
            "Both sides are numbers, so they can be divided directly.",
        ));
    }

    None
}

fn explain_power_rule(
    left: &AlgebraExpr,
    right: &AlgebraExpr,
    before: &str,
    after: &str,
) -> Option<AlgebraExplanationStep> {
    if right.is_zero() {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Zero exponent rule",
            "Any non-zero expression raised to the power of zero equals one.",
        ));
    }

    if right.is_one() {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Power of one rule",
            "Any expression raised to the power of one remains unchanged.",
        ));
    }

    if left.is_one() {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "One base rule",
            "One raised to any power remains one.",
        ));
    }

    if left.is_zero() {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Zero base rule",
            "Zero raised to a positive power remains zero.",
        ));
    }

    if is_numeric(left) && is_numeric(right) {
        return Some(AlgebraExplanationStep::new(
            before,
            after,
            "Numeric power",
            "Both base and exponent are numbers, so the power can be calculated directly.",
        ));
    }

    None
}

fn is_numeric(expr: &AlgebraExpr) -> bool {
    matches!(expr, AlgebraExpr::Number(_))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::parse_algebra_expression;

    fn explain_text(source: &str) -> AlgebraExplanation {
        let expr = parse_algebra_expression(source).unwrap();
        explain_simplification(expr)
    }

    #[test]
    fn explains_additive_identity() {
        let explanation = explain_text("x + 0");

        assert_eq!(explanation.simplified, "x");
        assert!(explanation
            .steps
            .iter()
            .any(|step| step.rule == "Additive identity"));
    }

    #[test]
    fn explains_multiplicative_identity() {
        let explanation = explain_text("1 * x");

        assert_eq!(explanation.simplified, "x");
        assert!(explanation
            .steps
            .iter()
            .any(|step| step.rule == "Multiplicative identity"));
    }

    #[test]
    fn explains_self_subtraction() {
        let explanation = explain_text("x - x");

        assert_eq!(explanation.simplified, "0");
        assert!(explanation
            .steps
            .iter()
            .any(|step| step.rule == "Self subtraction"));
    }

    #[test]
    fn explains_combine_like_terms() {
        let explanation = explain_text("x + x");

        assert_eq!(explanation.simplified, "(2 * x)");
        assert!(explanation
            .steps
            .iter()
            .any(|step| step.rule == "Combine like terms"));
    }

    #[test]
    fn explains_numeric_expression() {
        let explanation = explain_text("2 + 3 * 4");

        assert_eq!(explanation.simplified, "14");
        assert!(!explanation.steps.is_empty());
    }
}
