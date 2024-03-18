#![allow(dead_code)]

use std::io;
use piglog::prelude::*;
use piglog::*;
use fspp::*;

use crate::places;
use crate::generation;
use crate::library::*;
use crate::config;
use crate::system;

// Constants
const DEFAULT_USER_GEN: &str =
"# --------------------- #
#    Generation File    #
# --------------------- #

# Other generation files to import. (Example: intensive_apps -> ~/.config/rebos/imports/intensive_apps.toml)
imports = [
    # \"intensive_apps\",
]

# Packages to be installed via the native package manager.
pkg_managers.system.pkgs = [
    # \"git\",
]

# Packages to be installed via Flatpak.
pkg_managers.flatpak.pkgs = [
    # \"com.github.tchx84.Flatseal\",
]

# Packages to be installed via Cargo.
pkg_managers.cargo.pkgs = [
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

plural_name = \"system packages\"

# ------------------------------- #
#    Additional configuration.    #
# ------------------------------- #

# many_pkg_args = BOOL: Can you supply many packages as an argument? Example: 'sudo apt install git vim wget'

config = { many_pkg_args = true }
";

const DEFAULT_FLATPAK_MANAGER_CONFIG: &str =
"# Flatpak

install = \"flatpak install #:?\"
remove = \"flatpak uninstall #:?\"
upgrade = \"flatpak upgrade\"

plural_name = \"flatpaks\"

config = { many_pkg_args = true }
";

const DEFAULT_CARGO_MANAGER_CONFIG: &str =
"# Cargo

install = \"cargo install #:?\"
remove = \"cargo uninstall #:?\"

plural_name = \"crates\"

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
        places::base_user().add_str("machines").add_str(&system_hostname),
        places::base_user().add_str("imports"),
        places::base_user().add_str("hooks"),
        places::base_user().add_str("pkg_managers"),
    ];

    for i in directories.iter() {
        if i.exists() == false {
            match directory::create(i) {
                Ok(_o) => info!("Created directory: {}", i.to_string()),
                Err(e) => {
                    error!("Failed to create directory: {}", i.to_string());
                    return Err(e);
                },
            };
        }
    }

    let files = vec![
        (DEFAULT_USER_GEN, config::config_for(Config::Generation, ConfigSide::User)),
        (DEFAULT_USER_GEN, places::base_user().add_str("machines").add_str(&system_hostname).add_str("gen.toml")),
        (DEFAULT_PACKAGE_MANAGER_CONFIG, places::base_user().add_str("pkg_managers/system.toml")),
        (DEFAULT_FLATPAK_MANAGER_CONFIG, places::base_user().add_str("pkg_managers/flatpak.toml")),
        (DEFAULT_CARGO_MANAGER_CONFIG, places::base_user().add_str("pkg_managers/cargo.toml")),
    ];

    for i in files.iter() {
        if i.1.exists() == false {
            match file::write(i.0, &i.1) {
                Ok(_o) => info!("Created file: {}", i.1.to_string()),
                Err(e) => {
                    error!("Failed to create file: {}", i.1.to_string());
                    return Err(e);
                },
            };
        }
    }

    Ok(())
}

// Return path for a config file.
pub fn config_for(config: Config, side: ConfigSide) -> Path {
    return match config {
        Config::Generation => match side {
            ConfigSide::User => places::base_user().add_str("gen.toml"),
            ConfigSide::System => match generation::current_gen() {
                Ok(o) => o,
                Err(_e) => {
                    error!("Failed to get config path for system generation!");
                    abort();
                    return Path::new("This should never get returned.");
                },
            },
        },
    };
}
