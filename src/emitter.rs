use core::panic;
use std::{fs::{File, OpenOptions}, io::Write};

pub struct Emitter {
    full_path: String,
    header: String,
    code: String,
}

impl Emitter {
    pub fn new(full_path: String) -> Self {
        Self {
            full_path,
            header: String::new(),
            code:   String::new(),
        }
    }

    pub fn emit(&mut self, code: &str) {
        self.code.push_str(code);
    }

    pub fn emit_line(&mut self, code: &str) {
        self.code.push_str(&format!("{}\n", code).to_string());
    }

    pub fn header_line(&mut self, code: &str) {
        self.header.push_str(&format!("{}\n", code).to_string());
    }

    pub fn write_file(&self) {
        match OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.full_path.clone())
        {
            Ok(mut file) => {
                file.write(format!("{}{}", self.header, self.code).as_bytes());
            },
            Err(e) => {
                panic!("Could not open {} for writing: {}", self.full_path, e);
            }
        };
    }
}