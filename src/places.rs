#![allow(dead_code)]

use crate::filesystem::*;
use crate::log::*;
use crate::{info, error};
use crate::dir;
use std::io;

// The setup function for the directories.
pub fn setup() -> Result<(), io::Error> {
    let directories = vec![
        base(),
        gens(),
    ];

    for i in directories.iter() {
        match create_directory(i) {
            Ok(_o) => info!("Created directory: {}", i),
            Err(e) => {
                error!("Failed to create directory: {}", i);
                return Err(e);
            },
        };
    }

    return Ok(());
}



// The base directory of operations for Dister.
pub fn base() -> String {
    return format!("{}/.dister-base", dir::home());
}

// The directory of generations.
pub fn gens() -> String {
    return format!("{}/generations", base());
}

// User's Dister config directory.
pub fn base_user() -> String {
    return format!("{}/dister", dir::config());
}
