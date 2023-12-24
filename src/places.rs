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



/// The base directory of operations for Dister.
pub fn base() -> Path {
    return dir::home().add_str(".dister-base");
}

/// The directory of generations.
pub fn gens() -> Path {
    return base().add_str("generations");
}

/// User's Dister config directory.
pub fn base_user() -> Path {
    return dir::config().add_str("dister");
}
