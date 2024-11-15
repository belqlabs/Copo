use std::error::Error;

use crate::entities::types::CopoEntity;

#[derive(Debug)]
pub enum CopoDefinitionError {
    CouldReadCopoDef(String),
    BadCopoDef(String),
}

impl std::fmt::Display for CopoDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_strs = match self {
            CopoDefinitionError::CouldReadCopoDef(msg) => [
                "CopoDefinitionError",
                msg.as_str(),
                "Check if the copo.toml exists in pwd and with copo has permition to read it.",
            ],
            CopoDefinitionError::BadCopoDef(msg) => [
                "BadCopoDef",
                msg.as_str(),
                "See copo.toml definition in documentation.",
            ],
        };

        write!(
            f,
            "[DEFINITION ERROR] -> {}\n  {}\n  [HINT] {}",
            error_strs[0], error_strs[1], error_strs[2]
        )
    }
}

impl Error for CopoDefinitionError {}

pub type CopoDefinitionResult<T> = Result<T, CopoDefinitionError>;
