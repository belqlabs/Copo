use std::collections::HashMap;

use crate::commom::types::ContextDefinition;

#[derive(Debug)]
pub struct ContextRecord {
    content: Vec<u8>,
    rw_lock: bool,
}

#[derive(Debug)]
pub struct Context {
    pub max_records: u32,
    pub headers: Vec<String>,
    pub records: HashMap<u32, ContextRecord>,
}

impl Context {
    pub fn new(ctx_def: ContextDefinition) -> Self {
        Self {
            max_records: ctx_def.max_records,
            headers: ctx_def.headers.clone(),
            records: HashMap::new(),
        }
    }
}
