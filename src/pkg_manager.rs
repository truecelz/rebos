#![allow(dead_code)]

use std::io;
use crate::package_management::PackageManager;
use crate::filesystem::*;
use crate::convert::*;
use crate::places;
use crate::library::*;
use crate::log::*;
use crate::error;

enum PackageManagementMode {
    Install,
    Remove,
    Sync,
    Upgrade,
}

pub fn install(pkgs: &Vec<String>) -> Result<(), io::Error> {
    return package_management(PackageManagementMode::Install, &pkgs);
}

pub fn uninstall(pkgs: &Vec<String>) -> Result<(), io::Error> {
    return package_management(PackageManagementMode::Remove, &pkgs);
}

pub fn sync() -> Result<(), io::Error> {
    return package_management(PackageManagementMode::Sync, &vec![String::from("A")]);
}

pub fn upgrade() -> Result<(), io::Error> {
    return package_management(PackageManagementMode::Upgrade, &vec![String::from("A")]);
}

fn get_package_manager() -> Result<PackageManager, io::Error> {
    let mut package_manager: PackageManager = match toml::from_str(match read_file(format!("{}/pkg_manager.toml", places::base_user()).as_str()) {
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

    package_manager.set_plural_name("packages");

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

    if pkg_stuff.len() == 0 {
        return Ok(());
    }

    if pkg_stuff[0].trim() == "" {
        return Ok(());
    }

    for i in pkg_stuff.iter() {
        let pmm_result = match pmm {
            PackageManagementMode::Install => package_manager.install(i),
            PackageManagementMode::Remove => package_manager.remove(i),
            PackageManagementMode::Sync => package_manager.sync(),
            PackageManagementMode::Upgrade => package_manager.upgrade(),
        };

        match pmm_result {
            Ok(_o) => {},
            Err(e) => return Err(e),
        };
    }

    return Ok(());
}
