use std::{fs, path::PathBuf, str::FromStr};

use home::home_dir;

use crate::{
    commom::{
        errors::{CopoFileError, CopoFileErrors, CopoFileResult},
        types::{CopoFiles, ProcessDir},
    },
    entities::types::CopoEntity,
};

pub struct FileMaker {
    pub application_name: String,
    pub processes_names: Vec<String>,
    dot_copo_path: Option<PathBuf>,
    application_path: Option<PathBuf>,
}

impl FileMaker {
    pub fn new(application_name: &String, processes_names: &Vec<String>) -> Self {
        Self {
            application_name: application_name.to_string(),
            processes_names: processes_names.clone(),
            dot_copo_path: None,
            application_path: None,
        }
    }

    fn path_exists_and_is_dir(&self, path: &PathBuf, entity: CopoEntity) -> CopoFileResult<bool> {
        let path_exists = match fs::exists(&path) {
            Ok(exist) => exist,
            Err(_) => {
                return Err(CopoFileError {
                    entity,
                    error: CopoFileErrors::CouldNotCheckFile(path.to_string_lossy().to_string()),
                })
            }
        };

        if !path_exists {
            return Ok(false);
        }

        let path_is_dir = match fs::metadata(&path) {
            Ok(meta) => meta.is_dir(),
            Err(_) => {
                return Err(CopoFileError {
                    entity,
                    error: CopoFileErrors::CouldNotCheckFile(path.to_string_lossy().to_string()),
                })
            }
        };

        Ok(path_is_dir)
    }

    fn create_path_for_name(
        &self,
        parent: &Option<PathBuf>,
        entity_name: &String,
    ) -> CopoFileResult<PathBuf> {
        let parent_path: PathBuf = match parent {
            Some(dir) => dir.clone(),
            None => {
                return Err(CopoFileError {
                    entity: CopoEntity::Application,
                    error: CopoFileErrors::CouldNotCheckFile(
                        format!("Could not find the parent paht to {}. This is probably a bug. Please report it!", entity_name).to_string(),
                    ),
                })
            }
        };

        let entity_path = match PathBuf::from_str(entity_name.as_str()) {
            Ok(dir) => dir,
            Err(_) => {
                return Err(CopoFileError {
                    entity: CopoEntity::Application,
                    error: CopoFileErrors::CouldNotCreatePathFromStr(format!(
                        "You provided {}. If you think this is a bug, please report it.",
                        entity_name
                    )),
                })
            }
        };

        Ok(parent_path.join(entity_path))
    }

    fn create_dir_if_dont_exists(
        &self,
        path: &PathBuf,
        entity: CopoEntity,
    ) -> CopoFileResult<PathBuf> {
        let path_exists_and_is_dir = self.path_exists_and_is_dir(&path, entity.clone())?;

        if path_exists_and_is_dir {
            Ok(path.to_path_buf())
        } else {
            match fs::create_dir(path) {
                Ok(_) => Ok(path.to_path_buf()),
                Err(_) => Err(CopoFileError {
                    entity,
                    error: CopoFileErrors::CouldNotCreateFile(path.to_string_lossy().to_string()),
                }),
            }
        }
    }

    fn create_file_if_dont_exists(
        &self,
        path: &PathBuf,
        entity: CopoEntity,
    ) -> CopoFileResult<PathBuf> {
        match fs::File::create(path) {
            Ok(_) => Ok(path.to_path_buf()),
            Err(_) => Err(CopoFileError {
                entity,
                error: CopoFileErrors::CouldNotCreateFile(path.to_string_lossy().to_string()),
            }),
        }
    }

    fn create_or_return_dot_copo(&self) -> CopoFileResult<PathBuf> {
        let dot_copo_dir = self.create_path_for_name(&home_dir(), &".copo".to_string())?;

        self.create_dir_if_dont_exists(&dot_copo_dir, CopoEntity::Application)
    }

    fn create_or_return_application_dir(&self) -> CopoFileResult<PathBuf> {
        let application_dir_path =
            self.create_path_for_name(&self.dot_copo_path, &self.application_name)?;

        self.create_dir_if_dont_exists(&application_dir_path, CopoEntity::Application)
    }

    fn create_or_clear_pids_dir(&self) -> CopoFileResult<PathBuf> {
        let application_pids_path =
            self.create_path_for_name(&self.application_path, &"pids".to_string())?;

        let _ = self.create_dir_if_dont_exists(&application_pids_path, CopoEntity::Application)?;

        let dir_entries = match fs::read_dir(&application_pids_path) {
            Ok(iter) => iter,
            Err(_) => {
                return Err(CopoFileError {
                    entity: CopoEntity::Application,
                    error: CopoFileErrors::CouldNotReadDir(
                        application_pids_path.to_string_lossy().to_string(),
                    ),
                });
            }
        };

        for pid_dir_entry in dir_entries {
            let entry = match pid_dir_entry {
                Ok(entry) => entry,
                Err(_) => {
                    return Err(CopoFileError {
                        entity: CopoEntity::Application,
                        error: CopoFileErrors::CouldNotReadDir(
                            application_pids_path.to_string_lossy().to_string(),
                        ),
                    });
                }
            };

            let entry_meta = match fs::metadata(entry.path()) {
                Ok(entry_meta) => entry_meta,
                Err(_) => {
                    return Err(CopoFileError {
                        entity: CopoEntity::Application,
                        error: CopoFileErrors::CouldNotCheckFile(
                            entry.path().to_string_lossy().to_string(),
                        ),
                    })
                }
            };

            if entry_meta.is_file() {
                match fs::remove_file(entry.path()) {
                    Ok(_) => (),
                    Err(_) => {
                        return Err(CopoFileError {
                            entity: CopoEntity::Application,
                            error: CopoFileErrors::CouldNotRemoveFile(
                                entry.path().to_string_lossy().to_string(),
                            ),
                        });
                    }
                };
            } else if entry_meta.is_dir() {
                match fs::remove_dir_all(entry.path()) {
                    Ok(_) => (),
                    Err(_) => {
                        return Err(CopoFileError {
                            entity: CopoEntity::Application,
                            error: CopoFileErrors::CouldNotRemoveDir(
                                entry.path().to_string_lossy().to_string(),
                            ),
                        });
                    }
                }
            }
        }

        Ok(application_pids_path)
    }

    fn create_or_return_logs_dir(&self) -> CopoFileResult<PathBuf> {
        let logs_path = self.create_path_for_name(&self.application_path, &"logs".to_string())?;

        self.create_dir_if_dont_exists(&logs_path, CopoEntity::Application)
    }

    fn create_or_return_orchestrator_dir(&self) -> CopoFileResult<PathBuf> {
        let orchestrator_path =
            self.create_path_for_name(&self.application_path, &"orchestrator".to_string())?;

        self.create_dir_if_dont_exists(&orchestrator_path, CopoEntity::Orchestrator)
    }

    fn create_process_stdio_or_return(
        &self,
        process_path: &Option<PathBuf>,
    ) -> CopoFileResult<[PathBuf; 3]> {
        let process_stdio_path = self.create_path_for_name(process_path, &"stdio".to_string())?;

        let process_stdio_path_in_fs =
            Some(self.create_dir_if_dont_exists(&process_stdio_path, CopoEntity::Process)?);

        let stdin_path =
            self.create_path_for_name(&process_stdio_path_in_fs, &"stdin".to_string())?;

        let stdout_path =
            self.create_path_for_name(&process_stdio_path_in_fs, &"stdout".to_string())?;

        let stderr_path =
            self.create_path_for_name(&process_stdio_path_in_fs, &"stderr".to_string())?;

        Ok([stdin_path, stdout_path, stderr_path])
    }

    fn create_process(&self, process_name: &String) -> CopoFileResult<ProcessDir> {
        let process_path = self.create_path_for_name(&self.application_path, process_name)?;

        let process_path_in_fs =
            self.create_dir_if_dont_exists(&process_path, CopoEntity::Process)?;

        let stdio_paths =
            self.create_process_stdio_or_return(&Some(process_path_in_fs.to_path_buf()))?;

        Ok(ProcessDir {
            prc_name: process_name.to_string(),
            prc_dir: process_path_in_fs,
            prc_stdin: stdio_paths[0].to_path_buf(),
            prc_stdout: stdio_paths[1].to_path_buf(),
            prc_stderr: stdio_paths[1].to_path_buf(),
        })
    }

    pub fn create_copo_files(&mut self) -> CopoFileResult<CopoFiles> {
        self.dot_copo_path = Some(self.create_or_return_dot_copo()?);

        self.application_path = Some(self.create_or_return_application_dir()?);

        let app_reports_path =
            self.create_path_for_name(&self.application_path, &"reports.json".to_string())?;

        let _ = self.create_file_if_dont_exists(&app_reports_path, CopoEntity::Application);

        let logs_path = self.create_or_return_logs_dir()?;

        let pids_path = self.create_or_clear_pids_dir()?;

        let ocr_path = self.create_or_return_orchestrator_dir()?;

        let mut prc_paths: Vec<ProcessDir> = vec![];

        for prc_name in self.processes_names.iter() {
            let new_prc = self.create_process(prc_name)?;
            prc_paths.push(new_prc);
        }

        Ok(CopoFiles {
            dot_copo_path: self.dot_copo_path.clone().unwrap(),
            app_path: self.application_path.clone().unwrap(),
            app_reports_path,
            pids_path,
            logs_path,
            ocr_path,
            prc_paths,
        })
    }
}
