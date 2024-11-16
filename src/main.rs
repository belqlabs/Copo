mod commom;
mod entities;
mod utils;
use entities::{orchestrator, Orchestrator, Process};
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

    for prc_dir in file_making_result.prc_paths {
        let prc_def = match definition
            .processes
            .iter()
            .find(|prc| prc.name == prc_dir.prc_name)
        {
            Some(def) => def,
            None => todo!(),
        };

        processes.push(Process::new(prc_def.clone(), &prc_dir));
    }

    let mut orchestrator =
        Orchestrator::new(processes, file_making_result.pids_path, definition.context);

    match orchestrator.spawn("Node test".to_string()) {
        Ok(_) => (),
        Err(e) => show_err(Box::from(e), 1),
    };

    match orchestrator.spawn("Node test 2".to_string()) {
        Ok(_) => (),
        Err(e) => show_err(Box::from(e), 1),
    };
}
