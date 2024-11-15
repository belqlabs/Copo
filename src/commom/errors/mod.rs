// All errors offers ["ErrorName", "Error Message", Some(Hint)]
// The Hint is what a user can do to solve the error

pub mod copo_definition_errors;
pub mod copo_file_errors;
pub mod xos_errors;
pub use copo_definition_errors::*;
pub use copo_file_errors::*;
pub use xos_errors::*;
