use numora::config::LanguageConfig;
use numora::runtime::Runtime;

fn runtime() -> Runtime {
    Runtime::new(LanguageConfig::default())
}

#[test]
fn evaluates_basic_arithmetic() {
    let rt = runtime();

    let output = rt.run("1 + 2 * 3").unwrap();

    assert_eq!(output, "result: 7");
}

#[test]
fn evaluates_builtin_constants() {
    let rt = runtime();

    let output = rt.run("PI * 2").unwrap();

    assert!(output.starts_with("result: 6.283185307179586"));
}

#[test]
fn evaluates_builtin_functions() {
    let rt = runtime();

    let output = rt.run("sumof(1, 2, 3, 4)").unwrap();

    assert_eq!(output, "result: 10");
}

#[test]
fn evaluates_variables_program() {
    let rt = runtime();

    let source = r#"
@run calculator

given:
    n = 7

formula:
    f = (n + 1) / 4

find:
    f
"#;

    let output = rt.run(source).unwrap();

    assert_eq!(output, "result: f = 2");
}

#[test]
fn evaluates_steps_program() {
    let rt = runtime();

    let source = r#"
@run steps

given:
    n = 7

formula:
    f = (n + 1) / 4

find:
    f
"#;

    let output = rt.run(source).unwrap();

    assert!(output.contains("result: f = 2"));
    assert!(output.contains("steps:"));
    assert!(output.contains("f = ((7 + 1) / 4)"));
}

#[test]
fn solves_linear_equation() {
    let rt = runtime();

    let source = r#"
@run solve

equation:
    x + 3 = 10

solve:
    x
"#;

    let output = rt.run(source).unwrap();

    assert_eq!(output, "result: x = 7");
}

#[test]
fn solves_triangle_equation_positive_root() {
    let rt = runtime();

    let source = r#"
@run solve

given:
    a = 3
    b = 4

equation:
    a^2 + b^2 = c^2

solve:
    c
"#;

    let output = rt.run(source).unwrap();

    assert_eq!(output, "result: c = 5");
}

#[test]
fn evaluates_area_with_units() {
    let rt = runtime();

    let source = r#"
@run calculator

given:
    length = 5 m
    width = 4 m

formula:
    area = length * width

find:
    area
"#;

    let output = rt.run(source).unwrap();

    assert_eq!(output, "result: area = 20 m^2");
}

#[test]
fn evaluates_speed_with_units() {
    let rt = runtime();

    let source = r#"
@run calculator

given:
    distance = 100 m
    time = 20 s

formula:
    speed = distance / time

find:
    speed
"#;

    let output = rt.run(source).unwrap();

    assert_eq!(output, "result: speed = 5 m/s");
}

#[test]
fn rejects_invalid_unit_addition() {
    let rt = runtime();

    let source = r#"
@run calculator

given:
    length = 5 m
    time = 2 s

formula:
    bad = length + time

find:
    bad
"#;

    let error = rt.run(source).unwrap_err();

    assert!(error
        .to_string()
        .contains("Cannot add values with different units"));
}
