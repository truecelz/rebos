#![allow(dead_code)]

use std::io;
use crate::convert::*;
use crate::package_management::PackageManager;

enum Cargo {
    Install(Vec<String>),
    Remove(Vec<String>),
}

pub fn install(pkgs: &Vec<String>) -> Result<(), io::Error> {
    return cargo(Cargo::Install(pkgs.clone()));
}

pub fn uninstall(pkgs: &Vec<String>) -> Result<(), io::Error> {
    return cargo(Cargo::Remove(pkgs.clone()));
}

fn cargo(mode: Cargo) -> Result<(), io::Error> {
    #[allow(unreachable_patterns)]
    match mode {
        Cargo::Install(ref p) => {
            if p.len() == 0 {
                return Ok(());
            }

            if p[0].trim() == "" {
                return Ok(());
            }
        },
        Cargo::Remove(ref p) => {
            if p.len() == 0 {
                return Ok(());
            }

            if p[0].trim() == "" {
                return Ok(());
            }
        },
        _ => {}, // If the Cargo mode does not have a package list, don't do the ZERO check.
    };

    let cargo_manager: PackageManager = get_cargo_manager();

    match mode {
        Cargo::Install(pkgs) => {
            let pkgs_string = string_vec_to_string(&pkgs, " ");

            match cargo_manager.install(pkgs_string.as_str()) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };
        },
        Cargo::Remove(pkgs) => {
            let pkgs_string = string_vec_to_string(&pkgs, " ");

            match cargo_manager.remove(pkgs_string.as_str()) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };
        },
    };

    return Ok(());
}

fn get_cargo_manager() -> PackageManager {
    return PackageManager {
        install: String::from("cargo install #:?"),
        remove: String::from("cargo uninstall #:?"),
        plural_name: String::from("crates"),
        ..PackageManager::default()
    };
}
