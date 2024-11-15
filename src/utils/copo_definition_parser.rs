use std::fs::{self};

use crate::commom::{
    errors::{CopoDefinitionError, CopoDefinitionResult},
    types::CopoDefinition,
};

pub fn parse_definition() -> CopoDefinitionResult<CopoDefinition> {
    let definition_str = match fs::read_to_string("copo.toml") {
        Ok(def_str) => def_str,
        Err(e) => return Err(CopoDefinitionError::CouldReadCopoDef(e.to_string())),
    };

    let definition = match toml::from_str(&definition_str) {
        Ok(def) => def,
        Err(e) => return Err(CopoDefinitionError::BadCopoDef(e.to_string())),
    };

    Ok(definition)
}
