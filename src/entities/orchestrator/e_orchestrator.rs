use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::{Child, Command},
};

use crate::{
    commom::types::ContextDefinition,
    entities::{Context, PrcStdio, Process},
};

use super::{OrchestratorError, OrchestratorErrors, OrchestratorResult};

#[derive(Debug)]
pub struct OrchestratorChild {
    prc_name: String,
    prc_pid: u32,
    os_prc: Child,
}

#[derive(Debug)]
pub struct Orchestrator {
    processes: Vec<Process>,
    children: Vec<OrchestratorChild>,
    pids_path: PathBuf,
    context: Context,
}

impl Orchestrator {
    fn create_file_to_prc(
        &self,
        prc_name: String,
        path: &PathBuf,
        op: &str,
    ) -> OrchestratorResult<File> {
        match op {
            "read" => match File::open(path) {
                Ok(f) => Ok(f),
                Err(e) => Err(OrchestratorError {
                    prc_name,
                    error: OrchestratorErrors::CouldNotCreateReadOnlyFile(e.to_string()),
                }),
            },
            "write" => match File::create(path) {
                Ok(f) => Ok(f),
                Err(e) => Err(OrchestratorError {
                    prc_name,
                    error: OrchestratorErrors::CouldNotCreateWriteOnlyFile(e.to_string()),
                }),
            },
            _ => Err(OrchestratorError {
                prc_name,
                error: OrchestratorErrors::InvalidFileOperation(op.to_string()),
            }),
        }
    }

    fn create_prc_stdio_files(&self, prc: &Process) -> OrchestratorResult<PrcStdio<File>> {
        println!("{0:?}", prc);

        match prc.create_stdio_files() {
            Ok(_) => (),
            Err(e) => {
                return Err(OrchestratorError {
                    prc_name: prc.name.clone(),
                    error: OrchestratorErrors::CouldNotCreateStdio(e.to_string()),
                })
            }
        };

        let stdin =
            self.create_file_to_prc(prc.name.to_string(), &prc.prc_dir.prc_stdin, "read")?;

        let stdout =
            self.create_file_to_prc(prc.name.to_string(), &prc.prc_dir.prc_stdout, "write")?;

        let stderr =
            self.create_file_to_prc(prc.name.to_string(), &prc.prc_dir.prc_stderr, "write")?;

        Ok(PrcStdio {
            stdin,
            stdout,
            stderr,
        })
    }

    fn create_pid_file(&self, prc_name: String, prc_pid: u32) -> OrchestratorResult<()> {
        let mut prc_pid_path = self.pids_path.clone();

        prc_pid_path.push(format!("{}.pid", prc_name));

        let mut pid_file = match File::create_new(&prc_pid_path) {
            Ok(pf) => pf,
            Err(e) => {
                return Err(OrchestratorError {
                    prc_name,
                    error: OrchestratorErrors::CouldNotCreatePidFile(e.to_string()),
                })
            }
        };

        match pid_file.write(prc_pid.to_string().as_bytes()) {
            Ok(b) => b,
            Err(e) => {
                return Err(OrchestratorError {
                    prc_name,
                    error: OrchestratorErrors::CouldNotCreatePidFile(e.to_string()),
                })
            }
        };

        Ok(())
    }

    fn add_custom_args_to_command(
        &self,
        args: &Vec<String>,
        command: &mut Command,
    ) -> OrchestratorResult<()> {
        for arg in args.iter() {
            command.arg(arg);
        }

        Ok(())
    }

    pub fn new(processes: Vec<Process>, pids_path: PathBuf, ctx_def: ContextDefinition) -> Self {
        Self {
            processes,
            children: vec![],
            context: Context::new(ctx_def),
            pids_path,
        }
    }

    pub fn spawn(&mut self, prc_name: String) -> OrchestratorResult<()> {
        let mut frozen_prcs = self.processes.iter().clone();
        let prc_to_spawn = match frozen_prcs.find(|prc| prc.name == prc_name) {
            Some(prc) => prc,
            None => todo!(),
        };

        let prc_stdio = self.create_prc_stdio_files(prc_to_spawn)?;

        let mut new_command = Command::new(&prc_to_spawn.executable_path);

        new_command.arg(&prc_to_spawn.file_path);

        new_command.stdin(prc_stdio.stdin);

        new_command.stdout(prc_stdio.stdout);

        new_command.stderr(prc_stdio.stderr);

        let _ = self.add_custom_args_to_command(&prc_to_spawn.args, &mut new_command);

        let new_child = match new_command.spawn() {
            Ok(nc) => nc,
            Err(e) => {
                return Err(OrchestratorError {
                    prc_name: prc_to_spawn.name.clone(),
                    error: OrchestratorErrors::SpawnError(e.to_string()),
                })
            }
        };

        let _ = self.create_pid_file(prc_to_spawn.name.clone(), new_child.id());

        println!(
            "[ORCHESTRATOR] -> Spawned {} with PID: {}",
            prc_to_spawn.name,
            new_child.id()
        );

        self.children.push(OrchestratorChild {
            prc_name: prc_to_spawn.name.clone(),
            prc_pid: new_child.id(),
            os_prc: new_child,
        });

        Ok(())
    }
}
