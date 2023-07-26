#![allow(dead_code)]

use crate::filesystem::read_file;

// The base directory of operations for Dister.
pub fn base() -> String {
    return String::from("/etc/dister");
}

// The directory of generations.
pub fn gens() -> String {
    return format!("{}/generations", base());
}

// The current generation.
pub fn cgen() -> String {
    let current = read_file(format!("{}/current", gens()).as_str()).unwrap();
    let current = current.trim();

    return format!("{}/{}", gens(), current);
}
