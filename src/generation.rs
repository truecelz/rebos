#![allow(dead_code)]

use std::io;
use crate::log::*;
use crate::debug;

pub fn build() -> Result<(), io::Error> {
    debug!("Please work on generation::build()!");

    return Ok(());
}

pub fn rollback(by: usize) -> Result<(), io::Error> {
    debug!("Please work on generation::rollback()!");
    debug!("generation::rollback({})", by);

    return Ok(());
}
