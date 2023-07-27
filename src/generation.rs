#![allow(dead_code)]

use std::io;
use crate::filesystem::*;
use crate::places;
use crate::log::*;
use crate::debug;

pub fn commit() -> Result<(), io::Error> {
    debug!("Please work on generation::commit()!");

    return Ok(());
}

pub fn build() -> Result<(), io::Error> {
    debug!("Please work on generation::build()!");

    return Ok(());
}

pub fn rollback(by: usize) -> Result<(), io::Error> {
    debug!("Please work on generation::rollback()!");
    debug!("generation::rollback({})", by);

    return Ok(());
}

pub fn current() -> String {
    let current = read_file(format!("{}/current", places::gens()).as_str()).unwrap();
    let current = current.trim();

    return format!("{}/{}.toml", places::gens(), current);
}
