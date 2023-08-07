#![allow(dead_code)]

use std::io;
use crate::library::custom_error;
use crate::log::*;
use crate::error;

pub fn hostname() -> Result<String, io::Error> {
    return Ok(match hostname::get() {
        Ok(o) => match o.into_string() {
            Ok(o) => o,
            Err(_e) => {
                error!("Failed to parse hostname OsString into String type!");
                return Err(custom_error("Failed to parse OsString into String!"));
            },
        },
        Err(e) => {
            error!("Failed to get system hostname!");
            return Err(e);
        },
    });
}
