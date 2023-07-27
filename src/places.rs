#![allow(dead_code)]

use crate::filesystem::*;
use crate::log::*;
use crate::error;
use crate::dir;
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

// User's Dister config directory.
pub fn base_user() -> String {
    return format!("{}/dister", dir::config());
}
