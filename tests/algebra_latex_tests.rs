use numora::algebra::{parse_algebra_expression, simplify_expression, to_latex};

#[test]
fn renders_number_times_variable_without_dot() {
    let expr = parse_algebra_expression("2 * x").unwrap();

    assert_eq!(to_latex(&expr), "2x");
}

#[test]
fn renders_variable_times_number_as_coefficient_form() {
    let expr = parse_algebra_expression("x * 2").unwrap();

    assert_eq!(to_latex(&expr), "2x");
}

#[test]
fn renders_variable_times_variable_with_dot() {
    let expr = parse_algebra_expression("x * y").unwrap();

    assert_eq!(to_latex(&expr), "x \\cdot y");
}

#[test]
fn renders_fraction() {
    let expr = parse_algebra_expression("x / y").unwrap();

    assert_eq!(to_latex(&expr), "\\frac{x}{y}");
}

#[test]
fn renders_power() {
    let expr = parse_algebra_expression("x ^ 2").unwrap();

    assert_eq!(to_latex(&expr), "x^{2}");
}

#[test]
fn renders_power_with_grouped_base() {
    let expr = parse_algebra_expression("(x + 1) ^ 2").unwrap();

    assert_eq!(to_latex(&expr), "\\left(x + 1\\right)^{2}");
}

#[test]
fn renders_greek_theta() {
    let expr = parse_algebra_expression("theta").unwrap();

    assert_eq!(to_latex(&expr), "\\theta");
}

#[test]
fn renders_greek_lambda() {
    let expr = parse_algebra_expression("lambda").unwrap();

    assert_eq!(to_latex(&expr), "\\lambda");
}

#[test]
fn escapes_underscore_variable() {
    let expr = parse_algebra_expression("x_1").unwrap();

    assert_eq!(to_latex(&expr), "x\\_1");
}

#[test]
fn renders_simplified_expression_as_latex() {
    let expr = parse_algebra_expression("2 * x + 0").unwrap();
    let simplified = simplify_expression(expr.clone());

    assert_eq!(to_latex(&simplified), "2x");
}
