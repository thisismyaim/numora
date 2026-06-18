use numora::algebra::{explain_simplification, parse_algebra_expression};

#[test]
fn explanation_contains_top_level_latex_fields() {
    let expr = parse_algebra_expression("2 * x + 0").unwrap();

    let explanation = explain_simplification(expr);

    assert_eq!(explanation.original, "((2 * x) + 0)");
    assert_eq!(explanation.simplified, "(2 * x)");

    assert_eq!(explanation.latex_original, "2x + 0");
    assert_eq!(explanation.latex_simplified, "2x");
}

#[test]
fn explanation_steps_contain_latex_fields() {
    let expr = parse_algebra_expression("x + 0").unwrap();

    let explanation = explain_simplification(expr);

    assert!(!explanation.steps.is_empty());

    let first_step = &explanation.steps[0];

    assert_eq!(first_step.before, "(x + 0)");
    assert_eq!(first_step.after, "x");

    assert_eq!(first_step.latex_before, "x + 0");
    assert_eq!(first_step.latex_after, "x");

    assert_eq!(first_step.rule, "Additive identity");
}

#[test]
fn explanation_latex_supports_power_rule() {
    let expr = parse_algebra_expression("x ^ 1").unwrap();

    let explanation = explain_simplification(expr);

    assert_eq!(explanation.latex_original, "x^{1}");
    assert_eq!(explanation.latex_simplified, "x");

    assert!(explanation
        .steps
        .iter()
        .any(|step| step.rule == "Power of one rule"));

    assert!(explanation
        .steps
        .iter()
        .any(|step| step.latex_before == "x^{1}" && step.latex_after == "x"));
}

#[test]
fn explanation_latex_supports_grouped_power() {
    let expr = parse_algebra_expression("(x + 1) ^ 2").unwrap();

    let explanation = explain_simplification(expr);

    assert_eq!(explanation.latex_original, "\\left(x + 1\\right)^{2}");
}
