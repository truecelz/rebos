#![allow(dead_code)]

use std::io;
use crate::convert::*;
use crate::package_management::PackageManager;

enum Flatpak {
    Install(Vec<String>),
    Remove(Vec<String>),
    Upgrade,
}

pub fn install(pkgs: &Vec<String>) -> Result<(), io::Error> {
    return flatpak(Flatpak::Install(pkgs.clone()));
}

pub fn uninstall(pkgs: &Vec<String>) -> Result<(), io::Error> {
    return flatpak(Flatpak::Remove(pkgs.clone()));
}

pub fn upgrade() -> Result<(), io::Error> {
    return flatpak(Flatpak::Upgrade);
}

fn flatpak(mode: Flatpak) -> Result<(), io::Error> {
    match mode {
        Flatpak::Install(ref p) => {
            if p.len() == 0 {
                return Ok(());
            }

            if p[0].trim() == "" {
                return Ok(());
            }
        },
        Flatpak::Remove(ref p) => {
            if p.len() == 0 {
                return Ok(());
            }

            if p[0].trim() == "" {
                return Ok(());
            }
        },
        _ => {}, // If the Flatpak mode does not have a package list, don't do the ZERO check.
    };

    let flatpak_manager: PackageManager = get_flatpak_manager();

    match mode {
        Flatpak::Install(pkgs) => {
            let pkgs_string = string_vec_to_string(&pkgs, " ");

            match flatpak_manager.install(pkgs_string.as_str()) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };
        },
        Flatpak::Remove(pkgs) => {
            let pkgs_string = string_vec_to_string(&pkgs, " ");

            match flatpak_manager.remove(pkgs_string.as_str()) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };
        },
        Flatpak::Upgrade => match flatpak_manager.upgrade() {
            Ok(_o) => {},
            Err(e) => return Err(e),
        },
    };

    return Ok(());
}

fn get_flatpak_manager() -> PackageManager {
    return PackageManager {
        install: String::from("flatpak install #:?"),
        remove: String::from("flatpak uninstall #:?"),
        upgrade: String::from("flatpak upgrade"),
        plural_name: String::from("flatpaks"),
        ..PackageManager::default()
    };
}
