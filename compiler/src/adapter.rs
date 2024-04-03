use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

use parser::Program;

pub struct Adapter {
    pub source: String,
}

impl Adapter {
    pub fn load(path: &Path) -> Result<Self, Error> {
        if let Ok(content) = fs::read_to_string(path) {
            return Ok(Self { source: content });
        }

        Err(Error::new(ErrorKind::NotFound, "Adapter not found"))
    }

    pub fn run(&self, program: Program) {
        println!("{program:?}");
    }
}
