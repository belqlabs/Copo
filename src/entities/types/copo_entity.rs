#[derive(Debug, Clone)]
pub enum CopoEntity {
    Application,
    Orchestrator,
    Process,
}

impl CopoEntity {
    pub fn to_str(&self) -> &str {
        match self {
            CopoEntity::Application => "Application",
            CopoEntity::Orchestrator => "Orchestrator",
            CopoEntity::Process => "Process",
        }
    }
}

impl std::fmt::Display for CopoEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
