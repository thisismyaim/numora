use numora::runtime::Runtime;

#[test]
fn algebra_steps_output_contains_latex_fields() {
    let source = r#"
@run algebra steps

simplify:
    result = x + 0

find:
    result
"#;

    let output = Runtime::run_default(source).unwrap();
    // let output = runtime.run(source).unwrap();

    assert!(output.contains("result: result = x"));
    assert!(output.contains("latex: result = x"));

    assert!(output.contains("steps:"));
    assert!(output.contains("rule: Additive identity"));
    assert!(output.contains("latex_before: x + 0"));
    assert!(output.contains("latex_after: x"));
}
