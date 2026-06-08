use super::mode::ModeKind;
use super::registry::find_mode;

#[derive(Debug, Clone)]
pub struct ModePipeline {
    pub modes: Vec<String>,
}

impl ModePipeline {
    pub fn new(raw_modes: Vec<String>) -> Result<Self, String> {
        let normalized = normalize_modes(raw_modes)?;
        validate_modes(&normalized)?;

        Ok(Self {
            modes: normalized,
        })
    }
}

pub fn normalize_modes(raw_modes: Vec<String>) -> Result<Vec<String>, String> {
    if raw_modes.is_empty() {
        return Ok(vec!["calculator".to_string()]);
    }

    // V1 compatibility:
    // @run steps means @run calculator steps
    // @run summary means @run calculator summary
    if raw_modes.len() == 1 {
        let first = raw_modes[0].as_str();

        if first == "steps" || first == "summary" {
            return Ok(vec![
                "calculator".to_string(),
                first.to_string(),
            ]);
        }
    }

    Ok(raw_modes)
}

pub fn validate_modes(modes: &[String]) -> Result<(), String> {
    let mut has_executor = false;

    for (index, mode_name) in modes.iter().enumerate() {
        let Some(mode) = find_mode(mode_name) else {
            return Err(format!(
                "Unknown run mode: \"{}\".\n\nKnown modes:\n    calculator\n    physics\n    steps\n    summary",
                mode_name
            ));
        };

        match mode.kind {
            ModeKind::Executor => {
                if has_executor {
                    return Err(format!(
                        "Mode order error: multiple executor modes are not supported yet.\n\nProblem mode:\n    {}\n\nFor now, use one executor first, for example:\n    @run calculator steps\n    @run physics steps",
                        mode.name
                    ));
                }

                if index != 0 {
                    return Err(format!(
                        "Mode order error: executor mode \"{}\" must come before output modes.\n\nUse:\n    @run {} steps",
                        mode.name,
                        mode.name
                    ));
                }

                has_executor = true;
            }

            ModeKind::Explainer | ModeKind::Reporter => {
                if !has_executor {
                    return Err(format!(
                        "Mode order error: \"{}\" cannot run before an executor mode.\n\nUse:\n    @run calculator {}\n\nOr for physics:\n    @run physics {}",
                        mode.name,
                        mode.name,
                        mode.name
                    ));
                }
            }
        }
    }

    if !has_executor {
        return Err(
            "Mode pipeline error: no executor mode found.\n\nUse one executor mode first:\n    @run calculator steps\n    @run physics steps"
                .to_string(),
        );
    }

    Ok(())
}
