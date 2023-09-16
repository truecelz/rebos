#![allow(dead_code)]

use std::io;
use crate::places;
use crate::generation;
use crate::log::*;
use crate::{info, error};
use crate::filesystem::*;
use crate::library::*;
use crate::config;
use crate::system;

// Constants
const DEFAULT_USER_GEN: &str =
"# --------------------- #
#    Generation File    #
# --------------------- #

# Other generation files to import. (Example: intensive_apps -> ~/.config/dister/imports/intensive_apps.toml)
imports = [
    # \"intensive_apps\",
]

# Packages to be installed via the native package manager.
pkgs = [
    # \"git\",
]

# Packages to be installed via Flatpak.
flatpaks = [
    # \"com.github.tchx84.Flatseal\",
]

# Packages to be installed via Cargo.
crates = [
    # \"bacon\",
]
";

const DEFAULT_PACKAGE_MANAGER_CONFIG: &str =
"# ----------------------------------- #
#    Package Manager Configuration    #
# ----------------------------------- #

# Make sure to enter the exact command you use as the normal user!
# That means including 'sudo' or 'doas' or whatever if the command needs it.
# Where you would put packages, enter '#:?'.

# Example: install = \"sudo apt install #:?\"

install = \"\" # Example: sudo apt install #:?
remove = \"\" # Example: sudo apt remove #:?
sync = \"\" # Example: sudo apt update
upgrade = \"\" # Example: sudo apt upgrade

# ------------------------------- #
#    Additional configuration.    #
# ------------------------------- #

# many_pkg_args = BOOL: Can you supply many packages as an argument? Example: 'sudo apt install git vim wget'

config = { many_pkg_args = true }
";

// This determinds if a function should
// use the files from the user's config,
// or from the base() directory.
#[derive(PartialEq, Clone, Copy)]
pub enum ConfigSide {
    User,
    System,
}

// What to grab a config file for.
pub enum Config {
    Generation,
}

// Create the user configuration.
pub fn init_user_config() -> Result<(), io::Error> {
    let system_hostname = match system::hostname() {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let directories = vec![
        places::base_user(),
        format!("{}/machines/{}", places::base_user(), system_hostname),
        format!("{}/imports", places::base_user()),
    ];

    for i in directories.iter() {
        if path_exists(i) == false {
            match create_directory(i) {
                Ok(_o) => info!("Created directory: {}", i),
                Err(e) => {
                    error!("Failed to create directory: {}", i);
                    return Err(e);
                },
            };
        }
    }

    let files = vec![
        (DEFAULT_USER_GEN, config::config_for(Config::Generation, ConfigSide::User)),
        (DEFAULT_USER_GEN, format!("{}/machines/{}/gen.toml", places::base_user(), system_hostname)),
        (DEFAULT_PACKAGE_MANAGER_CONFIG, format!("{}/pkg_manager.toml", places::base_user())),
    ];

    for i in files.iter() {
        if path_exists(i.1.as_str()) == false {
            match write_file(i.0, i.1.as_str()) {
                Ok(_o) => info!("Created file: {}", i.1),
                Err(e) => {
                    error!("Failed to create file: {}", i.1);
                    return Err(e);
                },
            };
        }
    }

    return Ok(());
}

// Return path for a config file.
pub fn config_for(config: Config, side: ConfigSide) -> String {
    return match config {
        Config::Generation => match side {
            ConfigSide::User => format!("{}/gen.toml", places::base_user()),
            ConfigSide::System => match generation::current_gen() {
                Ok(o) => o,
                Err(_e) => {
                    error!("Failed to get config path for system generation!");
                    abort();
                    return String::from("Hidden string. UwU");
                },
            },
        },
    };
}
