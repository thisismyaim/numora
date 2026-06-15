#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeKind {
    Execution,
    Output,
}

#[derive(Debug, Clone)]
pub struct ModeInfo {
    pub name: String,
    pub kind: ModeKind,
}

impl ModeInfo {
    pub fn execution(name: &str) -> Self {
        Self {
            name: name.to_string(),
            kind: ModeKind::Execution,
        }
    }

    pub fn output(name: &str) -> Self {
        Self {
            name: name.to_string(),
            kind: ModeKind::Output,
        }
    }

    pub fn is_execution(&self) -> bool {
        self.kind == ModeKind::Execution
    }

    pub fn is_output(&self) -> bool {
        self.kind == ModeKind::Output
    }
}
