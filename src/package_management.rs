#![allow(dead_code)]

use std::collections::HashMap;
use std::io;
use serde::Deserialize;
use crate::library::*;
use crate::filesystem::*;
use crate::convert::*;
use crate::log::*;
use crate::places;
use crate::error;

#[derive(Deserialize, Debug)]
struct PackageManager {
    install: String,
    remove: String,
    sync: String,
    upgrade: String,
    config: HashMap<String, bool>,
}

impl PackageManagerFunctions for PackageManager {
    fn config_value(&self, key: &str) -> Result<bool, io::Error> {
        return match self.config.get(key) {
            Some(s) => Ok(*s),
            None => {
                error!("Missing keyword in package manager configuration! ({})", key);
                return Err(custom_error("Missing keyword in package manager config."));
            },
        };
    }
}

trait PackageManagerFunctions {
    fn config_value(&self, key: &str) -> Result<bool, io::Error>;
}

enum PackageManagementMode {
    Install,
    Remove,
    Sync,
    Upgrade,
}

pub fn install(pkgs: &Vec<String>) -> Result<(), io::Error> {
    return package_management(PackageManagementMode::Install, &pkgs);
}

pub fn remove(pkgs: &Vec<String>) -> Result<(), io::Error> {
    return package_management(PackageManagementMode::Remove, &pkgs);
}

pub fn sync() -> Result<(), io::Error> {
    return package_management(PackageManagementMode::Sync, &vec![String::new()]);
}

pub fn upgrade() -> Result<(), io::Error> {
    return package_management(PackageManagementMode::Upgrade, &vec![String::new()]);
}

fn get_package_manager() -> Result<PackageManager, io::Error> {
    let package_manager: PackageManager = match toml::from_str(match read_file(format!("{}/pkg_manager.toml", places::base_user()).as_str()) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to read package manager TOML file!");
            return Err(e);
        },
    }.as_str()) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to parse package manager TOML file!");
            error!("TOML Error: {:?}", e);

            return Err(custom_error("Failed to parse TOML file!"));
        },
    };

    return Ok(package_manager);
}

fn package_management(pmm: PackageManagementMode, pkgs: &Vec<String>) -> Result<(), io::Error> {
    let package_manager: PackageManager = match get_package_manager() {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let pkg_stuff = match match package_manager.config_value("many_pkg_args") {
        Ok(o) => o,
        Err(e) => return Err(e),
    } {
        true => vec![string_vec_to_string(&pkgs, " ")],
        false => pkgs.clone(),
    };

    for i in pkg_stuff.iter() {
        match run_command(match pmm {
            PackageManagementMode::Install => sed(package_manager.install.as_str(), "#:?", i),
            PackageManagementMode::Remove => sed(package_manager.remove.as_str(), "#:?", i),
            PackageManagementMode::Sync => package_manager.sync.to_string(),
            PackageManagementMode::Upgrade => package_manager.upgrade.to_string(),
        }.as_str()) {
            true => {},
            false => {
                error!("Failed to run command related to package management!");
                return Err(custom_error("Failed to run command related to package management!"));
            },
        };
    }

    return Ok(());
}
