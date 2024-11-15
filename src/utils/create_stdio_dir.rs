use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn create_stdio_dir(processes_names: &[&String]) -> PathBuf {
    let stdio_path = Path::new("./.copo/stdio");

    fs::create_dir_all(stdio_path).unwrap();

    for prc_name in processes_names.iter() {
        let prc_stdio_dir_path_str = format!("{}/{}", stdio_path.to_string_lossy(), prc_name);

        fs::create_dir(Path::new(&prc_stdio_dir_path_str)).unwrap();
    }

    stdio_path.to_path_buf()
}
