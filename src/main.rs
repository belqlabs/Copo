mod commom;
mod entities;
mod utils;
use entities::Process;
use utils::{parse_definition, show_err, FileMaker};

fn main() {
    let definition = match parse_definition() {
        Ok(d) => d,
        Err(e) => show_err(Box::from(e), 1),
    };

    let mut processes_names: Vec<String> = vec![];

    for prc in definition.processes.iter() {
        processes_names.push(prc.name.to_string());
    }

    let mut file_maker = FileMaker::new(&definition.application.name, &processes_names);

    let file_making_result = match file_maker.create_copo_files() {
        Ok(fmr) => fmr,
        Err(e) => show_err(Box::from(e), 1),
    };

    let mut processes: Vec<Process> = vec![];

    for prc_dir in file_making_result.prc_paths{
        let prc_def = definition.processes
    }
}
