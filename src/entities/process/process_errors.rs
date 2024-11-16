use std::error::Error;

#[derive(Debug)]
pub enum ProcessErrors {
    CouldNotCreate(String, String),
    UnkownStdioMember(String),
}

#[derive(Debug)]
pub struct ProcessError {
    pub prc_name: String,
    pub error: ProcessErrors,
}

impl std::fmt::Display for ProcessErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_strs = match self {
            ProcessErrors::CouldNotCreate(stdm, msg) => [
                format!("CouldNotCreate{}", stdm),
                msg.to_string(),
                "Check permision in the .copo directory".to_string(),
            ],
            ProcessErrors::UnkownStdioMember(msg) => [
                format!("UnkownStdioMember"),
                msg.to_string(),
                "This is probably a bug. Please report it.".to_string(),
            ],
        };

        write!(
            f,
            "[{}]\n  {}\n  [HINT] {}",
            err_strs[0], err_strs[1], err_strs[2]
        )
    }
}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[PROCESS ERROR] -> [PROCESS: {}]\n  {}",
            self.prc_name, self.error
        )
    }
}

impl Error for ProcessErrors {}
impl Error for ProcessError {}

pub type ProcessResult<T> = Result<T, ProcessError>;
