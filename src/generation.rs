#![allow(dead_code)]

use std::io;
use crate::filesystem::*;
use crate::places;
use crate::log::*;
use crate::{info, error, debug, generic};
use crate::library::*;
use crate::config;
use crate::config::{Config, ConfigSide};

// Get latest generation number.
fn latest_number() -> Result<usize, io::Error> {
    let list_len = match list() {
        Ok(o) => o,
        Err(e) => {
            return Err(e);
        },
    }.len();

    return Ok(list_len);
}

// Create a new system generation based on the user generation.
pub fn commit(msg: &str) -> Result<(), io::Error> {
    let gen_dir = format!("{}/{}", places::gens(), match latest_number() {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to get latest generation number!");
            return Err(e);
        },
    } + 1);

    match create_directory(gen_dir.as_str()) {
        Ok(_o) => info!("Created generation directory."),
        Err(e) => {
            error!("Failed to create generation directory!");
            return Err(e);
        },
    };

    let user_gen: String = match read_file(config::config_for(Config::Generation, ConfigSide::User).as_str()) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to read user generation file!");
            error!("Exact Error: {:?}", e);

            return Err(e);
        },
    };

    let files = vec![
        (msg, format!("{}/commit", gen_dir)),
        (user_gen.as_str(), format!("{}/gen.toml", gen_dir)),
    ];

    for i in files.iter() {
        match write_file(i.0, i.1.as_str()) {
            Ok(_o) => info!("Created file: {}", name_from_path(i.1.as_str())),
            Err(e) => {
                error!("Failed to create file: {}", i.1);
                return Err(e);
            },
        };
    }

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

    return format!("{}/{}/gen.toml", places::gens(), current);
}
