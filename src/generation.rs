#![allow(dead_code)]

use std::io;
use crate::filesystem::*;
use crate::places;
use crate::log::*;
use crate::{error, debug, generic};
use crate::library::*;

// Create a new system generation based on the user generation.
pub fn commit(msg: &str) -> Result<(), io::Error> {
    debug!("Please work on generation::commit()!");
    debug!("generation::commit(\"{}\")", msg);

    return Ok(());
}

// Build the 'current' system generation.
pub fn build() -> Result<(), io::Error> {
    debug!("Please work on generation::build()!");

    return Ok(());
}

// Set the 'current' generation to another older generation.
pub fn rollback(by: isize) -> Result<(), io::Error> {
    debug!("Please work on generation::rollback()!");
    debug!("generation::rollback({})", by);

    return Ok(());
}

// Set the 'current' generation to the latest generation.
pub fn latest() -> Result<(), io::Error> {
    debug!("Please work on generation::latest()!");

    return Ok(());
}

// Set the 'current' generation to a specific generation.
pub fn set_current(to: isize) -> Result<(), io::Error> {
    debug!("Please work on generation::set_current()!");
    debug!("generation::set_current({})", to);

    return Ok(());
}

// List all generations.
pub fn list() -> Result<Vec<(String, String)>, io::Error> {
    let generations = match list_directory(places::gens().as_str()) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to list the generations directory! ({})", places::gens());
            return Err(e);
        },
    };

    let mut gens_with_info: Vec<(String, String)> = Vec::new();

    for i in generations.iter() {
        let generation_name = name_from_path(i);
        let commit_msg = match read_file(format!("{}/commit", i).as_str()) {
            Ok(o) => o,
            Err(_e) => "<< COMMIT MESSAGE MISSING >>".to_string(),
        };

        gens_with_info.push((generation_name, commit_msg));
    }

    return Ok(gens_with_info);
}

// Print out the list of generations.
pub fn list_print() -> Result<(), io::Error> {
    let list_items = match list() {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    for i in list_items.iter() {
        generic!("{} ... ({})", i.0, i.1);
    }

    return Ok(());
}

// Get the 'current' generation TOML file.
pub fn get_current() -> String {
    let current = read_file(format!("{}/current", places::gens()).as_str()).unwrap();
    let current = current.trim();

    return format!("{}/{}.toml", places::gens(), current);
}
