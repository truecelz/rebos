#![allow(dead_code)]

use crate::filesystem::*;
use crate::log::*;
use crate::error;
use std::io;

// The setup function for the directories.
pub fn setup() -> Result<(), io::Error> {
    let mut results: Vec<Result<(), io::Error>> = Vec::new();

    results.push(create_directory(base().as_str()));
    results.push(create_directory(gens().as_str()));

    for i in results {
        match i {
            Ok(_o) => {},
            Err(e) => {
                error!("Failed to create core program directories!");
                return Err(e);
            },
        };
    }

    return Ok(());
}



// The base directory of operations for Dister.
pub fn base() -> String {
    return String::from("/etc/dister");
}

// The directory of generations.
pub fn gens() -> String {
    return format!("{}/generations", base());
}



// The current generation. (Not created at setup.)
pub fn cgen() -> String {
    let current = read_file(format!("{}/current", gens()).as_str()).unwrap();
    let current = current.trim();

    return format!("{}/{}", gens(), current);
}
