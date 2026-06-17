use numora::algebra::{explain_simplification, parse_algebra_expression};

#[test]
fn explanation_model_explains_add_zero() {
    let expr = parse_algebra_expression("x + 0").unwrap();
    let explanation = explain_simplification(expr);

    assert_eq!(explanation.original, "(x + 0)");
    assert_eq!(explanation.simplified, "x");
    assert!(explanation
        .steps
        .iter()
        .any(|step| step.rule == "Additive identity"));
}

#[test]
fn explanation_model_explains_like_terms() {
    let expr = parse_algebra_expression("2 * x + 3 * x").unwrap();
    let explanation = explain_simplification(expr);

    assert_eq!(explanation.simplified, "(5 * x)");
    assert!(explanation
        .steps
        .iter()
        .any(|step| step.rule == "Combine like terms"));
}

#[test]
fn explanation_model_explains_power_rule() {
    let expr = parse_algebra_expression("x ^ 1").unwrap();
    let explanation = explain_simplification(expr);

    assert_eq!(explanation.simplified, "x");
    assert!(explanation
        .steps
        .iter()
        .any(|step| step.rule == "Power of one rule"));
}
