#![allow(dead_code)]

use piglog::prelude::*;

use crate::generation::Generation;
use crate::obj_print_boilerplate::macros::*;

pub fn generation(gen: &Generation) {
    println!("");

    for i in gen.pkg_managers.keys() {
        print_entry!(i, gen.pkg_managers.get(i).unwrap().pkgs);
    }
}
