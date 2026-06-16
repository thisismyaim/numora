use crate::config::LanguageConfig;
use crate::environment::Environment;
use crate::error::Numora;
use crate::includes::IncludeResolver;
use crate::modes::{ModeExecutor, ModePipeline};
use crate::program::{detect_run_modes, evaluate_expression};

pub struct Runtime {
    config: LanguageConfig,
}

impl Runtime {
    pub fn new(config: LanguageConfig) -> Self {
        Self { config }
    }

    // Keep old V1/V2 call style:
    // let rt = Runtime::new(...);
    // rt.run("1 + 2")
    pub fn run(&self, source: &str) -> Result<String, Numora> {
        self.run_with_config(source)
    }

    // Optional helper if any code wants:
    // Runtime::run_default("1 + 2")
    pub fn run_default(source: &str) -> Result<String, Numora> {
        let runtime = Runtime::new(LanguageConfig::default());
        runtime.run(source)
    }

    pub fn run_with_config(&self, source: &str) -> Result<String, Numora> {
        self.validate_feature_access(source)?;

        if self.looks_like_direct_expression(source) {
            return self.run_direct_expression(source);
        }

        let expanded_source = IncludeResolver::expand_source(source)?;

        let requested_modes = detect_run_modes(&expanded_source);
        let pipeline = ModePipeline::new(requested_modes);
        let context = pipeline.build_context()?;

        ModeExecutor::execute(&expanded_source, &context)
    }

    fn validate_feature_access(&self, source: &str) -> Result<(), Numora> {
        if source.contains("unit:") && !self.config.units_enabled {
            return Err(Numora::EvaluationError(
                "Units feature is disabled".to_string(),
            ));
        }

        if source.contains("@run ide") && !self.config.ide_api_enabled {
            return Err(Numora::EvaluationError(
                "IDE API feature is disabled".to_string(),
            ));
        }

        Ok(())
    }

    fn looks_like_direct_expression(&self, source: &str) -> bool {
        let trimmed = source.trim();

        if trimmed.is_empty() {
            return false;
        }

        if trimmed.contains('\n') {
            return false;
        }

        if trimmed.starts_with("@run") {
            return false;
        }

        if trimmed.contains("given:") || trimmed.contains("formula:") || trimmed.contains("find:") {
            return false;
        }

        true
    }

    fn run_direct_expression(&self, source: &str) -> Result<String, Numora> {
        let mut environment = Environment::new();
        let value = evaluate_expression(source, &mut environment)?;

        Ok(format!("result: {}", format_number(value.number)))
    }
}

fn format_number(number: f64) -> String {
    if number.fract() == 0.0 {
        format!("{}", number as i64)
    } else {
        format!("{}", number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_expression_still_works() {
        let runtime = Runtime::new(LanguageConfig::default());
        let result = runtime.run("1 + 2 * 3").unwrap();

        assert!(result.contains("7"));
    }

    #[test]
    fn run_steps_keeps_v1_compatibility() {
        let runtime = Runtime::new(LanguageConfig::default());

        let source = r#"
@run steps

given:
    x = 1
    y = 2

formula:
    result = x + y

find:
    result
"#;

        let result = runtime.run(source).unwrap();

        assert!(
            result.contains("3")
                || result.to_lowercase().contains("step")
                || result.to_lowercase().contains("result")
        );
    }

    #[test]
    fn calculator_steps_is_valid() {
        let runtime = Runtime::new(LanguageConfig::default());

        let source = r#"
@run calculator steps

given:
    x = 1
    y = 2

formula:
    result = x + y

find:
    result
"#;

        assert!(runtime.run(source).is_ok());
    }

    #[test]
    fn physics_steps_is_valid() {
        let runtime = Runtime::new(LanguageConfig::default());

        let source = r#"
@run physics steps

given:
    x = 1
    y = 2

formula:
    result = x + y

find:
    result
"#;

        assert!(runtime.run(source).is_ok());
    }

    #[test]
    fn solve_is_valid() {
        let runtime = Runtime::new(LanguageConfig::default());

        let source = r#"
    @run solve

    given:
        x = 2

    equation:
        y = x + 3

    solve:
        y
    "#;

        assert!(runtime.run(source).is_ok());
    }

    #[test]
    fn steps_calculator_is_invalid() {
        let runtime = Runtime::new(LanguageConfig::default());

        let source = r#"
@run steps calculator

given:
    x = 1

formula:
    result = x

find:
    result
"#;

        assert!(runtime.run(source).is_err());
    }

    #[test]
    fn summary_calculator_is_invalid() {
        let runtime = Runtime::new(LanguageConfig::default());

        let source = r#"
@run summary calculator

given:
    x = 1

formula:
    result = x

find:
    result
"#;

        assert!(runtime.run(source).is_err());
    }

    #[test]
    fn unknown_mode_is_invalid() {
        let runtime = Runtime::new(LanguageConfig::default());

        let source = r#"
@run unknown

given:
    x = 1

formula:
    result = x

find:
    result
"#;

        let error = runtime.run(source).unwrap_err();
        let message = format!("{:?}", error);

        assert!(message.contains("Unknown run mode"));
    }
}

#[test]
fn algebra_is_valid() {
    let runtime = Runtime::new(LanguageConfig::default());

    let source = r#"
@run algebra

given:
    x = 2
    y = 3

formula:
    result = x + y

find:
    result
"#;

    let result = runtime.run(source).unwrap();

    assert!(result.contains("5"));
}

#[test]
fn algebra_steps_is_valid() {
    let runtime = Runtime::new(LanguageConfig::default());

    let source = r#"
@run algebra steps

given:
    x = 2
    y = 3

formula:
    result = x + y

find:
    result
"#;

    let result = runtime.run(source).unwrap();

    assert!(
        result.contains("5")
            || result.to_lowercase().contains("step")
            || result.to_lowercase().contains("result")
    );
}

#[test]
fn runtime_expands_include_file() {
    use std::fs;

    let runtime = Runtime::new(LanguageConfig::default());

    let mut dir = std::env::temp_dir();
    dir.push("numora_runtime_include_test");

    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    fs::write(
        dir.join("common.mth"),
        r#"
given:
    x = 2
    y = 3
"#,
    )
    .unwrap();

    let source = format!(
        r#"
@run calculator
@include "{}"

formula:
    result = x + y

find:
    result
"#,
        dir.join("common.mth").display()
    );

    let result = runtime.run(&source).unwrap();

    assert!(result.contains("5"));
}
