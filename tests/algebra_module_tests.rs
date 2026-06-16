use numora::algebra::{
    parse_algebra_expression, simplify_expression, solve_linear_equation, AlgebraExpr,
    LinearEquation,
};

#[test]
fn algebra_module_simplifies_basic_expression() {
    let expr = AlgebraExpr::add(AlgebraExpr::variable("x"), AlgebraExpr::number(0.0));

    let simplified = simplify_expression(expr);

    assert_eq!(simplified.to_string(), "x");
}

#[test]
fn algebra_module_parses_and_simplifies_expression() {
    let expr = parse_algebra_expression("x + 0").unwrap();
    let simplified = simplify_expression(expr);

    assert_eq!(simplified.to_string(), "x");
}

#[test]
fn algebra_module_parses_operator_precedence() {
    let expr = parse_algebra_expression("x + 2 * y").unwrap();

    assert_eq!(expr.to_string(), "(x + (2 * y))");
}

#[test]
fn algebra_module_solves_linear_equation() {
    let equation = LinearEquation::new("x", 2.0, 3.0, 11.0);

    let solution = solve_linear_equation(equation).unwrap();

    assert_eq!(solution, 4.0);
}
