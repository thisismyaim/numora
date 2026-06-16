use numora::config::LanguageConfig;
use numora::runtime::Runtime;

#[test]
fn algebra_simplify_example_file_runs() {
    let runtime = Runtime::new(LanguageConfig::default());

    let output = runtime
        .run_file("examples/projects/algebra_simplify_example.mth")
        .unwrap();

    assert!(output.contains("result: result = x"));
    assert!(output.contains("result: second = y"));
    assert!(output.contains("result: third = 14"));
    assert!(output.contains("steps:"));
}
