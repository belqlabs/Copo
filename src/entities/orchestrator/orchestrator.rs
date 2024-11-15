use std::process::Child;

use crate::entities::Process;

pub struct OrchestratorChild {
    prc_name: String,
    os_prc: Child,
}

pub struct Orchestrator {
    processes: Vec<Process>,
    children: Option<Vec<OrchestratorChild>>,
}

impl Orchestrator {
    pub fn new(processes: Vec<Process>) -> Self {
        Self {
            processes,
            children: None,
        }
    }
}
