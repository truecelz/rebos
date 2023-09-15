#![allow(dead_code)]

use crate::generation::Generation;
use crate::log::*;
use crate::{generic, info};
use crate::obj_print_boilerplate::macros::*;

pub fn generation(gen: &Generation) {
    println!("");

    print_entry!("Packages", gen.pkgs);
    print_entry!("Flatpaks", gen.flatpaks);
    print_entry!("Crates", gen.crates);
}
