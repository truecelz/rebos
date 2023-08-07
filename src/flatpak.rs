#![allow(dead_code)]

use std::io;
use crate::library::*;
use crate::convert::*;
use crate::log::*;
use crate::{info, error};

enum Flatpak {
    Install(Vec<String>),
    Remove(Vec<String>),
    Upgrade,
}

pub fn install(pkgs: &Vec<String>) -> Result<(), io::Error> {
    if pkgs.len() == 0 {
        return Ok(());
    }

    if pkgs[0].trim() == "" {
        return Ok(());
    }

    return flatpak(Flatpak::Install(pkgs.clone()));
}

pub fn uninstall(pkgs: &Vec<String>) -> Result<(), io::Error> {
    if pkgs.len() == 0 {
        return Ok(());
    }

    if pkgs[0].trim() == "" {
        return Ok(());
    }

    return flatpak(Flatpak::Remove(pkgs.clone()));
}

pub fn upgrade() -> Result<(), io::Error> {
    return flatpak(Flatpak::Upgrade);
}

fn flatpak(mode: Flatpak) -> Result<(), io::Error> {
    match mode {
        Flatpak::Install(pkgs) => {
            match run_command(format!("flatpak install {}", string_vec_to_string(&pkgs, " ")).as_str()) {
                true => info!("Installed Flatpaks successfully!"),
                false => {
                    error!("Failed to install Flatpaks!");
                    return Err(custom_error("Failed to install Flatpaks!"));
                },
            };
        },
        Flatpak::Remove(pkgs) => {
            match run_command(format!("flatpak uninstall {}", string_vec_to_string(&pkgs, " ")).as_str()) {
                true => info!("Removed Flatpaks successfully!"),
                false => {
                    error!("Failed to remove Flatpaks!");
                    return Err(custom_error("Failed to remove Flatpaks!"));
                },
            };
        },
        Flatpak::Upgrade => {
            match run_command("flatpak upgrade") {
                true => info!("Upgraded Flatpaks successfully!"),
                false => {
                    error!("Failed to upgrade Flatpaks!");
                    return Err(custom_error("Failed to upgrade Flatpaks!"));
                },
            };
        },
    };

    return Ok(());
}
