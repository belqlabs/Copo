use std::{error::Error, io};

#[derive(Debug)]
pub enum OrchestratorErrors {
    CouldNotCreateStdio(String),
    CouldNotCreateReadOnlyFile(String),
    CouldNotCreateWriteOnlyFile(String),
    CouldNotCreatePidFile(String),
    InvalidFileOperation(String),
    SpawnError(String),
}

#[derive(Debug)]
pub struct OrchestratorError {
    pub prc_name: String,
    pub error: OrchestratorErrors,
}

impl std::fmt::Display for OrchestratorErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_strs = match self {
            OrchestratorErrors::CouldNotCreateStdio(msg) => [
                "CouldNotCreateStdio",
                msg.as_str(),
                "Check permision in the .copo directory",
            ],
            OrchestratorErrors::InvalidFileOperation(msg) => [
                "InvalidFileOperation",
                msg.as_str(),
                "This is probably a bug. Please report it.",
            ],
            OrchestratorErrors::CouldNotCreateReadOnlyFile(msg) => [
                "CouldNotCreateReadOnlyFile",
                msg.as_str(),
                "Check permision in the .copo directory",
            ],
            OrchestratorErrors::CouldNotCreateWriteOnlyFile(msg) => [
                "CouldNotCreateWriteOnlyFile",
                msg.as_str(),
                "Check permision in the .copo directory",
            ],
            OrchestratorErrors::SpawnError(msg) => {
                ["SpawnError", msg.as_str(), "Read the system error."]
            }
            OrchestratorErrors::CouldNotCreatePidFile(msg) => [
                "CouldNotCreatePidFile",
                msg.as_str(),
                "If this error is happened because the pid file already existed, this is a bug. Please report it.",
            ],
        };

        write!(
            f,
            "[{}]\n  {}\n  [HINT] {}",
            err_strs[0], err_strs[1], err_strs[2]
        )
    }
}

impl std::fmt::Display for OrchestratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ORCHESTRATOR ERROR] -> [PROCESS: {}]\n  {}",
            self.prc_name, self.error
        )
    }
}

impl Error for OrchestratorErrors {}
impl Error for OrchestratorError {}

pub type OrchestratorResult<T> = Result<T, OrchestratorError>;
