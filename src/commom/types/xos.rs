use crate::commom::errors::*;
use std::{env::consts, error::Error, process::Command};

pub struct XOsCommands {
    terminate: Option<Vec<String>>,
    kill: Option<Vec<String>>,
}

pub struct XOs {
    pub os_name: String,
    commands: XOsCommands,
}

impl XOsCommands {
    pub fn new(os: &str) -> Self {
        let x_terminate = match os {
            "windows" => Some(vec![
                "taskkill".to_string(),
                "/t".to_string(),
                "/pid".to_string(),
            ]),
            "linux" => Some(vec!["kill".to_string()]),
            _ => None,
        };

        let x_kill = match os {
            "windows" => Some(vec![
                "taskill".to_string(),
                "/t".to_string(),
                "/f".to_string(),
                "/pid".to_string(),
            ]),
            "linux" => Some(vec!["kill".to_string(), "-l".to_string(), "9".to_string()]),
            _ => None,
        };

        Self {
            terminate: x_terminate,
            kill: x_kill,
        }
    }
}

impl XOs {
    pub fn new() -> Self {
        Self {
            os_name: consts::OS.to_string(),
            commands: XOsCommands::new(consts::OS),
        }
    }

    pub fn terminate(&self, pid: u32) -> XOSResult<bool> {
        let command_args = match &self.commands.terminate {
            Some(commands) => commands,
            None => {
                return Err(XOSError {
                    command: "terminate".to_string(),
                    error: XOSErrors::OSNotDefined(self.os_name.to_string()),
                })
            }
        };

        let mut command = Command::new(&command_args[0]);

        for arg_idx in 1..command_args.len() {
            command.arg(&command_args[arg_idx]);
        }

        command.arg(pid.to_string());

        let command_output = match command.output() {
            Ok(otp) => match String::from_utf8(otp.stdout) {
                Ok(output) => output,
                Err(e) => {
                    return Err(XOSError {
                        command: "terminate".to_string(),
                        error: XOSErrors::CouldNotParseOutput(e.to_string()),
                    })
                }
            },
            Err(e) => {
                return Err(XOSError {
                    command: "terminate".to_string(),
                    error: XOSErrors::CouldNotTerminate(e.to_string()),
                })
            }
        };

        let command_result = match self.os_name.as_str() {
            "windows" => command_output.contains(&"SUCCESS".to_string()),
            "linux" => command_output.contains(&"terminated".to_string()),
            _ => {
                return Err(XOSError {
                    command: "terminate".to_string(),
                    error: XOSErrors::OSNotDefined(self.os_name.to_string()),
                })
            }
        };

        Ok(command_result)
    }

    pub fn kill(&self, pid: u32) -> XOSResult<bool> {
        let command_args = match &self.commands.kill {
            Some(commands) => commands,
            None => {
                return Err(XOSError {
                    command: "terminate".to_string(),
                    error: XOSErrors::OSNotDefined(self.os_name.to_string()),
                })
            }
        };

        let mut command = Command::new(&command_args[0]);

        for arg_idx in 1..command_args.len() {
            command.arg(&command_args[arg_idx]);
        }

        command.arg(pid.to_string());

        match command.output() {
            Ok(otp) => match String::from_utf8(otp.stdout) {
                Ok(_) => (),
                Err(e) => {
                    return Err(XOSError {
                        command: "terminate".to_string(),
                        error: XOSErrors::CouldNotParseOutput(e.to_string()),
                    })
                }
            },
            Err(e) => {
                return Err(XOSError {
                    command: "terminate".to_string(),
                    error: XOSErrors::CouldNotTerminate(e.to_string()),
                })
            }
        };

        Ok(true)
    }
}
