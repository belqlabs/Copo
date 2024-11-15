use std::error::Error;

#[derive(Debug)]
pub enum XOSErrors {
    CouldNotKill(String),
    CouldNotTerminate(String),
    CouldNotParseOutput(String),
    OSNotDefined(String),
}

#[derive(Debug)]
pub struct XOSError {
    pub command: String,
    pub error: XOSErrors,
}

impl std::fmt::Display for XOSErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_name_strs = match self {
            XOSErrors::CouldNotKill(msg) => [
                "CouldNotKill",
                msg.as_str(),
                "Check the permitions of the user who spawned copo.",
            ],
            XOSErrors::CouldNotTerminate(msg) => [
                "CouldNotTerminate",
                msg.as_str(),
                "Check the permitions of the user who spawned copo.",
            ],
            XOSErrors::OSNotDefined(msg) => [
                "OSNotDefined",
                msg.as_str(),
                "Copo does not support your OS yet. Open an issue if you want it to do so.",
            ],
            XOSErrors::CouldNotParseOutput(msg) => [
                "CouldNotParseOutput",
                msg.as_str(),
                "This is propably happened because the stdout of the process received some UTF-8 invalid byte. If you thinks its a bug, report in the Copo repository.",
            ],
        };

        write!(
            f,
            "  [{}]: {}\n  [HINT]:{}",
            error_name_strs[0], error_name_strs[1], error_name_strs[2]
        )
    }
}

impl std::fmt::Display for XOSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[XOS ERROR] -> {}\n{}", self.command, self.error)
    }
}

impl Error for XOSErrors {}

impl Error for XOSError {}

pub type XOSResult<T> = Result<T, XOSError>;
