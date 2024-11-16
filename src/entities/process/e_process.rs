use std::{
    fs::{self, File},
    path::PathBuf,
};

use crate::commom::types::{ProcessDefinition, ProcessDir};

use super::{ProcessError, ProcessErrors, ProcessResult};

#[derive(Debug)]
pub struct PrcStdio<T> {
    pub stdout: T,
    pub stdin: T,
    pub stderr: T,
}

#[derive(Debug)]
pub struct Process {
    pub name: String,
    pub executable_path: String,
    pub prc_dir: ProcessDir,
    pub file_path: String,
    pub args: Vec<String>,
    pub depends_on: Vec<String>,
    pub trigger_type: String,
    pub trygger_definition: String,
}

impl Process {
    fn create_stdio_member_file(&self, stdm_name: &str, stdm_path: &PathBuf) -> ProcessResult<()> {
        match stdm_name {
            "stdin" => (),
            "stdout" => (),
            "stderr" => (),
            _ => {
                return Err(ProcessError {
                    prc_name: self.name.clone(),
                    error: ProcessErrors::UnkownStdioMember(stdm_name.to_string()),
                })
            }
        };

        match fs::exists(stdm_path) {
            Ok(exist) => {
                if exist {
                    match fs::remove_file(stdm_path) {
                        Ok(_) => (),
                        Err(e) => {
                            return Err(ProcessError {
                                prc_name: self.name.clone(),
                                error: ProcessErrors::CouldNotCreate(
                                    stdm_name.to_string(),
                                    e.to_string(),
                                ),
                            })
                        }
                    }
                }
            }
            Err(e) => {
                return Err(ProcessError {
                    prc_name: self.name.clone(),
                    error: ProcessErrors::CouldNotCreate(stdm_name.to_string(), e.to_string()),
                })
            }
        };

        match File::create(stdm_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProcessError {
                prc_name: self.name.clone(),
                error: ProcessErrors::CouldNotCreate(stdm_name.to_string(), e.to_string()),
            }),
        }
    }

    pub fn create_stdio_files(&self) -> ProcessResult<()> {
        self.create_stdio_member_file("stdin", &self.prc_dir.prc_stdin)?;

        self.create_stdio_member_file("stdout", &self.prc_dir.prc_stdout)?;

        self.create_stdio_member_file("stderr", &self.prc_dir.prc_stderr)?;

        Ok(())
    }

    pub fn new(prc_def: ProcessDefinition, prc_dir: &ProcessDir) -> Self {
        Self {
            name: prc_def.name,
            executable_path: prc_def.executable_path,
            file_path: prc_def.file_path,
            args: prc_def.args.clone(),
            prc_dir: prc_dir.clone(),
            depends_on: prc_def.depends_on.clone(),
            trigger_type: prc_def.trigger_type.to_string(),
            trygger_definition: prc_def.trygger_definition,
        }
    }
}
