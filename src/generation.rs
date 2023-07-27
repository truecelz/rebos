#![allow(dead_code)]

use std::io;
use crate::filesystem::*;
use crate::places;
use crate::log::*;
use crate::debug;

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
pub fn list() -> Result<Vec<String>, io::Error> {
    debug!("Please work on generation::list()!");

    return Ok(vec![String::from("Some generation example.")]);
}

// Get the 'current' generation TOML file.
pub fn get_current() -> String {
    let current = read_file(format!("{}/current", places::gens()).as_str()).unwrap();
    let current = current.trim();

    return format!("{}/{}.toml", places::gens(), current);
}
