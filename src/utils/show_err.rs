use std::{error::Error, process::exit};

pub fn show_err(error: Box<dyn Error>, code: i32) -> ! {
    eprintln!("{}", error);
    exit(code);
}
