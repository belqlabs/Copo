use std::error::Error;

use crate::entities::types::CopoEntity;

#[derive(Debug)]
// The (String) will hold the path that generated the error
pub enum CopoFileErrors {
    CouldNotCheckFile(String),
    CouldNotOpenFile(String),
    CouldNotReadDir(String),
    CouldNotCreateFile(String),
    CouldNotReadFile(String),
    CouldNotWriteFile(String),
    CouldNotReadPid(String),
    CouldNotCreatePathFromStr(String),
    CouldNotRemoveFile(String),
    CouldNotRemoveDir(String),
}

#[derive(Debug)]
pub struct CopoFileError {
    pub entity: CopoEntity,
    pub error: CopoFileErrors,
}

impl std::fmt::Display for CopoFileErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_name_strs = match self {
            CopoFileErrors::CouldNotCheckFile(msg) => [
                "CouldNotCheckFile",
                msg.as_str(),
                "Check if Copo created the file and copo's permisions in the path.",
            ],
            CopoFileErrors::CouldNotOpenFile(msg) => [
                "CouldNotOpenFile",
                msg.as_str(),
                "Check if Copo created the file and copo's permisions in the path.",
            ],
            CopoFileErrors::CouldNotCreateFile(msg) => [
                "CouldNotCreateFile",
                msg.as_str(),
                "Check if Copo created the file and copo's permisions in the path.",
            ],
            CopoFileErrors::CouldNotReadFile(msg) => [
                "CouldNotReadFile",
                msg.as_str(),
                "Check if Copo created the file and if copo has permisions read it.",
            ],
            CopoFileErrors::CouldNotWriteFile(msg) => [
                "CouldNotWriteFile",
                msg.as_str(),
                "Check if Copo created the file and if copo has permisions write in it.",
            ],
            CopoFileErrors::CouldNotReadDir(msg) => [
                "CouldNotReadDir",
                msg.as_str(),
                "Check if the directory exists and if copo has permisions read it.",
            ],
            CopoFileErrors::CouldNotReadPid(msg) => [
                "CouldNotReadPid",
                msg.as_str(),
                "This usually happens when the pid file was manually edited.",
            ],
            CopoFileErrors::CouldNotCreatePathFromStr(msg) => [
                "CouldNotCreatePathFromStr",
                msg.as_str(),
                "The name you provided for the entity could not be parsed to a valid path in your OS. Please change it.",
            ],
            CopoFileErrors::CouldNotRemoveFile(msg) => [
                "CouldNotRemoveFile",
                msg.as_str(),
                "Check if the file exists and if copo has permisions remove it.",
            ],
            CopoFileErrors::CouldNotRemoveDir(msg) => [
                "CouldNotRemoveDir",
                msg.as_str(),
                "Check if the directory exists and if copo has permisions remove it.",
            ],
        };

        write!(
            f,
            "  [{}]: {}\n  [HINT]:{}",
            error_name_strs[0], error_name_strs[1], error_name_strs[2]
        )
    }
}

impl std::fmt::Display for CopoFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[COPO FILE ERROR] -> {}\n{}",
            self.entity.to_str(),
            self.error
        )
    }
}

impl Error for CopoFileError {}
impl Error for CopoFileErrors {}

pub type CopoFileResult<T> = Result<T, CopoFileError>;
