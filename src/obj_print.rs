#![allow(dead_code)]

use crate::generation::Generation;
use crate::log::*;
use crate::{generic, info};

pub fn generation(gen: &Generation) {
    info!("Packages:");
    for i in gen.pkgs.iter() {
        generic!("{}", i);
    }

    info!("Flatpaks:");
    for i in gen.flatpaks.iter() {
        generic!("{}", i);
    }

    info!("Flatpak Repos:");
    for i in gen.flatpak_repos.iter() {
        generic!("Name: {}, URL: {}", i.0, i.1);
    }
}
