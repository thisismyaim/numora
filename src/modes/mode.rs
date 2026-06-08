#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeKind {
    Executor,
    Explainer,
    Reporter,
}

#[derive(Debug, Clone)]
pub struct ModeSpec {
    pub name: &'static str,
    pub kind: ModeKind,
    pub description: &'static str,
}

impl ModeSpec {
    pub fn is_executor(&self) -> bool {
        self.kind == ModeKind::Executor
    }

    pub fn is_output_mode(&self) -> bool {
        matches!(self.kind, ModeKind::Explainer | ModeKind::Reporter)
    }
}
