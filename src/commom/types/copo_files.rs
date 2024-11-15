use std::path::PathBuf;

#[derive(Debug)]
pub struct ProcessDir {
    pub prc_name: String,
    pub prc_dir: PathBuf,
    pub prc_stdin: PathBuf,
    pub src_stdout: PathBuf,
    pub src_stderr: PathBuf,
}

#[derive(Debug)]
pub struct CopoFiles {
    pub dot_copo_path: PathBuf,
    pub app_path: PathBuf,
    pub app_reports_path: PathBuf,
    pub pids_path: PathBuf,
    pub logs_path: PathBuf,
    pub ocr_path: PathBuf,
    pub prc_paths: Vec<ProcessDir>,
}
