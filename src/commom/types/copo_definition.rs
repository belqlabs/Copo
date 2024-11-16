use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationDefinition {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProcessDefinition {
    pub name: String,
    pub executable_path: String,
    pub args: Vec<String>,
    pub file_path: String,
    pub depends_on: Vec<String>,
    pub trigger_type: String,
    pub trygger_definition: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContextDefinition {
    pub max_records: u32,
    pub headers: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CopoDefinition {
    pub application: ApplicationDefinition,
    pub processes: Vec<ProcessDefinition>,
    pub context: ContextDefinition,
}
