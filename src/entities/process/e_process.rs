use std::fs::File;

use crate::commom::types::{ProcessDefinition, ProcessDir};

pub struct PrcStdio {
    pub stdout: File,
    pub stdin: File,
    pub stderr: File,
}

pub struct Process {
    pub name: String,
    pub executable_path: String,
    pub prc_dir: ProcessDir,
    pub file_path: String,
    pub depends_on: Vec<String>,
    pub trigger_type: String,
    pub trygger_definition: String,
    pub stdio: Option<PrcStdio>,
}

impl Process {
    pub fn new(prc_def: ProcessDefinition, prc_dir: ProcessDir) -> Self {
        Self {
            name: prc_def.name,
            executable_path: prc_def.executable_path,
            file_path: prc_def.file_path,
            prc_dir,
            depends_on: prc_def.depends_on.clone(),
            trigger_type: prc_def.trigger_type.to_string(),
            trygger_definition: prc_def.trygger_definition,
            stdio: None,
        }
    }
}
