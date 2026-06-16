use crate::modes::mode::{Mode, ModeCategory};

pub struct ModeRegistry;

impl ModeRegistry {
    pub fn get(name: &str) -> Option<Mode> {
        match name {
            "calculator" => Some(Mode::new("calculator", ModeCategory::Execution)),
            "algebra" => Some(Mode::new("algebra", ModeCategory::Execution)),
            "physics" => Some(Mode::new("physics", ModeCategory::Execution)),
            "solve" => Some(Mode::new("solve", ModeCategory::Execution)),

            "steps" => Some(Mode::new("steps", ModeCategory::Output)),
            "summary" => Some(Mode::new("summary", ModeCategory::Output)),

            _ => None,
        }
    }

    pub fn is_known(name: &str) -> bool {
        Self::get(name).is_some()
    }

    pub fn is_execution(name: &str) -> bool {
        Self::get(name)
            .map(|mode| mode.is_execution())
            .unwrap_or(false)
    }

    pub fn is_output(name: &str) -> bool {
        Self::get(name)
            .map(|mode| mode.is_output())
            .unwrap_or(false)
    }
}
