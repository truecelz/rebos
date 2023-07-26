#![allow(dead_code)]

use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

// Read a file.
pub fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Ok(_o) => {},
        Err(e) => return Err(e),
    };

    return Ok(file_contents);
}

// Write a file.
pub fn write_file(path: &str, contents: &str) -> Result<(), io::Error> {
    let mut file = match File::create(path) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    match file.write_all(contents.as_bytes()) {
        Ok(_o) => {},
        Err(e) => return Err(e),
    };

    return Ok(());
}

// Return a list of items in a directory. (Returns full paths!)
pub fn read_directory(directory: &str) -> Result<Vec<String>, io::Error> {
    let contents = match fs::read_dir(directory) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let mut return_contents: Vec<String> = Vec::new();

    for i in contents {
        return_contents.push(i.unwrap().path().display().to_string());
    }

    return Ok(return_contents);
}