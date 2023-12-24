#![allow(dead_code)]

use std::io;
use piglog::prelude::*;
use piglog::*;
use fspp::*;

use crate::dir;

// The setup function for the directories.
pub fn setup() -> Result<(), io::Error> {
    let directories = vec![
        base(),
        gens(),
    ];

    for i in directories.iter() {
        match directory::create(i) {
            Ok(_o) => info!("Created directory: {}", i.to_string()),
            Err(e) => {
                error!("Failed to create directory: {}", i.to_string());
                return Err(e);
            },
        };
    }

    return Ok(());
}



/// The base directory of operations for Rebos.
pub fn base() -> Path {
    return dir::home().add_str(".rebos-base");
}

/// The directory of generations.
pub fn gens() -> Path {
    return base().add_str("generations");
}

/// User's Rebos config directory.
pub fn base_user() -> Path {
    return dir::config().add_str("rebos");
}
