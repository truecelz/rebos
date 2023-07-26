#![allow(dead_code)]

use crate::log::*;
use crate::error;

pub enum ErrorType {
    RunAsRoot,
}

pub fn error_type(et: ErrorType) {
    match et {
        ErrorType::RunAsRoot => error!("That command must be run as root! Please use 'sudo' or switch to root!"),
    };
}
