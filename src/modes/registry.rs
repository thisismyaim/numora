use super::mode::{ModeKind, ModeSpec};

pub fn all_modes() -> Vec<ModeSpec> {
    vec![
        ModeSpec {
            name: "calculator",
            kind: ModeKind::Executor,
            description: "Executes normal math formulas, arithmetic, functions, and unit-aware calculations.",
        },
        ModeSpec {
            name: "physics",
            kind: ModeKind::Executor,
            description: "Executes physics formulas and physics-specific functions.",
        },
        ModeSpec {
            name: "steps",
            kind: ModeKind::Explainer,
            description: "Shows step-by-step explanation after an executor has produced a result.",
        },
        ModeSpec {
            name: "summary",
            kind: ModeKind::Reporter,
            description: "Shows a short summary after an executor has produced a result.",
        },
    ]
}

pub fn find_mode(name: &str) -> Option<ModeSpec> {
    all_modes()
        .into_iter()
        .find(|mode| mode.name == name)
}
