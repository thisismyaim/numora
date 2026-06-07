use crate::config::LanguageConfig;
use crate::environment::Environment;
use crate::error::Numora;
use crate::program::{detect_run_mode, evaluate_expression, run_math_program, RunMode};

pub struct Runtime {
    config: LanguageConfig,
}

impl Runtime {
    pub fn new(config: LanguageConfig) -> Self {
        Self { config }
    }

    pub fn run(&self, source: &str) -> Result<String, Numora> {
        if !self.config.calculator_enabled {
            return Err(Numora::EvaluationError(
                "Calculator feature is disabled".to_string(),
            ));
        }

        let trimmed = source.trim();

        if self.looks_like_math_program(trimmed) {
            self.validate_feature_access(trimmed)?;
            return run_math_program(trimmed);
        }

        let env = Environment::new();
        let result = evaluate_expression(trimmed, &env)?;

        Ok(format!("result: {}", result.format()))
    }

    fn validate_feature_access(&self, source: &str) -> Result<(), Numora> {
        let mode = detect_run_mode(source);

        match mode {
            RunMode::Calculator => {
                if source.contains("given:") && !self.config.variables_enabled {
                    return Err(Numora::EvaluationError(
                        "Variables feature is disabled".to_string(),
                    ));
                }
            }

            RunMode::Steps => {
                if !self.config.steps_enabled {
                    return Err(Numora::EvaluationError(
                        "Steps feature is disabled".to_string(),
                    ));
                }

                if source.contains("given:") && !self.config.variables_enabled {
                    return Err(Numora::EvaluationError(
                        "Variables feature is disabled".to_string(),
                    ));
                }
            }

            RunMode::Solve => {
                if !self.config.equations_enabled {
                    return Err(Numora::EvaluationError(
                        "Equations feature is disabled".to_string(),
                    ));
                }

                if source.contains("given:") && !self.config.variables_enabled {
                    return Err(Numora::EvaluationError(
                        "Variables feature is disabled".to_string(),
                    ));
                }
            }
        }

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

    fn looks_like_math_program(&self, source: &str) -> bool {
        source.contains("@run calculator")
            || source.contains("@run steps")
            || source.contains("@run solve")
            || source.contains("given:")
            || source.contains("formula:")
            || source.contains("equation:")
            || source.contains("find:")
            || source.contains("solve:")
            || source.contains("input:")
            || source.contains("unit:")
            || source.contains("@run ide")
    }
}
