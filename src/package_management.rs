#![allow(dead_code)]

use std::collections::HashMap;
use std::io;
use serde::Deserialize;
use crate::library::*;
use crate::filesystem::*;
use crate::convert::*;
use crate::log::*;
use crate::places;
use crate::{info, error};

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct PackageManager {
    pub install: String,
    pub remove: String,
    pub sync: String,
    pub upgrade: String,
    pub config: HashMap<String, bool>,
    pub plural_name: String,
}

impl PackageManager {
    pub fn config_value(&self, key: &str) -> Result<bool, io::Error> {
        return match self.config.get(key) {
            Some(s) => Ok(*s),
            None => {
                error!("Missing keyword in package manager configuration! ({})", key);
                return Err(custom_error("Missing keyword in package manager config!"));
            },
        };
    }

    pub fn install(&self, pkgs: &str) -> Result<(), io::Error> {
        match run_command(sed(self.install.as_str(), "#:?", pkgs).as_str()) {
            true => info!("Successfully installed {}!", self.plural_name),
            false => {
                error!("Failed to install {}!", self.plural_name);

                return Err(custom_error(format!("Failed to install {}!", self.plural_name).as_str()));
            },
        };

        return Ok(());
    }

    pub fn remove(&self, pkgs: &str) -> Result<(), io::Error> {
        match run_command(sed(self.remove.as_str(), "#:?", pkgs).as_str()) {
            true => info!("Successfully removed {}!", self.plural_name),
            false => {
                error!("Failed to remove {}!", self.plural_name);

                return Err(custom_error(format!("Failed to remove {}!", self.plural_name).as_str()));
            },
        };

        return Ok(());
    }

    pub fn sync(&self) -> Result<(), io::Error> {
        match run_command(self.sync.as_str()) {
            true => info!("Successfully installed repositories!"),
            false => {
                error!("Failed to sync repositories!");

                return Err(custom_error("Failed to sync repositories!"));
            },
        };

        return Ok(());
    }

    pub fn upgrade(&self) -> Result<(), io::Error> {
        match run_command(self.upgrade.as_str()) {
            true => info!("Successfully upgraded {}!", self.plural_name),
            false => {
                error!("Failed to upgrade {}!", self.plural_name);

                return Err(custom_error(format!("Failed to upgrade {}!", self.plural_name).as_str()));
            },
        };

        return Ok(());
    }

    pub fn set_plural_name(&mut self, pn: &str) {
        self.plural_name = pn.to_string();
    }

    fn return_config_hashmap_default() -> HashMap<String, bool> {
        let mut hm: HashMap<String, bool> = HashMap::new();

        hm.insert("many_pkg_args".to_string(), true);

        return hm;
    }

    pub fn default() -> Self {
        return Self {
            install: String::new(),
            remove: String::new(),
            sync: String::new(),
            upgrade: String::new(),
            config: Self::return_config_hashmap_default(),
            plural_name: String::from("PACKAGES FROM UNKNOWN PACKAGE MANAGER"),
        };
    }
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
