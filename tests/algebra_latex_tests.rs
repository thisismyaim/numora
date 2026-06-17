use numora::algebra::{parse_algebra_expression, simplify_expression, to_latex};

fn latex(source: &str) -> String {
    let expr = parse_algebra_expression(source).unwrap();
    to_latex(&expr)
}

fn simplified_latex(source: &str) -> String {
    let expr = parse_algebra_expression(source).unwrap();
    let simplified = simplify_expression(expr);
    to_latex(&simplified)
}

#[test]
fn latex_renders_basic_power() {
    assert_eq!(latex("x ^ 2"), "x^{2}");
}

#[test]
fn latex_renders_fraction() {
    assert_eq!(latex("x / y"), "\\frac{x}{y}");
}

#[test]
fn latex_renders_coefficient_multiplication() {
    assert_eq!(latex("2 * x"), "2x");
}

#[test]
fn latex_renders_simplified_like_terms() {
    assert_eq!(simplified_latex("2 * x + 3 * x"), "5x");
}

#[test]
fn latex_renders_simplified_power_rule() {
    assert_eq!(simplified_latex("x ^ 1"), "x");
}

#[test]
fn latex_renders_grouped_expression_power() {
    assert_eq!(latex("(x + 1) ^ 2"), "\\left(x + 1\\right)^{2}");
}
