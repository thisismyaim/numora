use crate::config::LanguageConfig;
use crate::environment::Environment;
use crate::error::Numora;
use crate::program::{evaluate_expression, run_math_program};

pub struct Runtime {
    config: LanguageConfig,
}

impl Runtime {
    pub fn new(config: LanguageConfig) -> Self {
        Self { config }
    }

    pub fn run(&self, source: &str) -> Result<String, Numora> {
        let source = source.trim();

        if source.is_empty() {
            return Err(Numora::ParserError("Empty input".to_string()));
        }

        self.validate_feature_access(source)?;

        if Self::looks_like_math_program_source(source) {
            run_math_program(source)
        } else {
            Self::evaluate_direct_expression(source)
        }
    }

    pub fn run_default(source: &str) -> Result<String, Numora> {
        let runtime = Self::new(LanguageConfig::default());
        runtime.run(source)
    }

    pub fn run_with_config(&self, source: &str) -> Result<String, Numora> {
        self.run(source)
    }

    fn evaluate_direct_expression(source: &str) -> Result<String, Numora> {
        let env = Environment::new();
        let value = evaluate_expression(source, &env)?;

        Ok(format!("result: {}", Self::format_number(value.number)))
    }

    fn format_number(number: f64) -> String {
        if number.fract() == 0.0 {
            format!("{}", number as i64)
        } else {
            format!("{}", number)
        }
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

    fn looks_like_math_program_source(source: &str) -> bool {
        source.lines().any(|line| {
            let line = line.trim();

            line.starts_with("@run")
                || line == "given:"
                || line == "formula:"
                || line == "equation:"
                || line == "find:"
                || line == "solve:"
        })
    }
}
