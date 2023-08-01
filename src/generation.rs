#![allow(dead_code)]

use std::io;
use colored::Colorize;

use serde::Deserialize;
use crate::filesystem::*;
use crate::places;
use crate::log::*;
use crate::{info, error, debug, generic};
use crate::library::*;
use crate::config;
use crate::config::{Config, ConfigSide};

// The structure for a generation.
#[derive(Deserialize, Debug)]
pub struct Generation {
    pub pkgs: Vec<String>,
    pub flatpaks: Vec<String>,
    pub flatpak_repos: Vec<(String, String)>,
}

// Get latest generation number.
pub fn latest_number() -> Result<usize, io::Error> {
    let gen_list = match list_with_no_calls() {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to get latest generation number!");
            return Err(e);
        },
    };

    let mut generation_numbers: Vec<usize> = Vec::new();

    for i in gen_list.iter() {
        generation_numbers.push(match i.0.trim().parse() {
            Ok(o) => o,
            Err(_e) => {
                error!("Tried to parse invalid generation name! ({})", i.0.trim());
                return Err(custom_error("Failed to parse invalid generation name!"));
            },
        });
    }

    if generation_numbers.len() < 1 {
        return Ok(0);
    }

    let latest_num = match generation_numbers.into_iter().max() {
        Some(s) => s,
        None => {
            error!("Failed to get max number in generation numbers list!");
            return Err(custom_error("Failed to get max number in generation number list!"));
        },
    };

    return Ok(latest_num);
}

// Create a new system generation based on the user generation.
pub fn commit(msg: &str) -> Result<(), io::Error> {
    let generation_number = match latest_number() {
        Ok(o) => o,
        Err(e) => return Err(e),
    } + 1;

    let gen_dir = format!("{}/{}", places::gens(), generation_number);

    match create_directory(gen_dir.as_str()) {
        Ok(_o) => info!("Created generation directory."),
        Err(e) => {
            error!("Failed to create generation directory!");
            return Err(e);
        },
    };

    let user_gen: String = match read_file(config::config_for(Config::Generation, ConfigSide::User).as_str()) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to read user generation file!");
            error!("Exact Error: {:?}", e);

            return Err(e);
        },
    };

    let files = vec![
        (msg, format!("{}/commit", gen_dir)),
        (user_gen.as_str(), format!("{}/gen.toml", gen_dir)),
    ];

    for i in files.iter() {
        match write_file(i.0, i.1.as_str()) {
            Ok(_o) => info!("Created file: {}", name_from_path(i.1.as_str())),
            Err(e) => {
                error!("Failed to create file: {}", i.1);
                return Err(e);
            },
        };
    }

    match set_current(generation_number) {
        Ok(_o) => {},
        Err(e) => return Err(e),
    };

    return Ok(());
}

// Build the 'current' system generation.
pub fn build() -> Result<(), io::Error> {
    debug!("Please work on generation::build()!");

    return Ok(());
}

// Set the 'current' generation to another older generation.
pub fn rollback(by: isize) -> Result<(), io::Error> {
    let current_num = match get_current() {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let new_current = (current_num as isize) - by;

    match set_current(new_current as usize) {
        Ok(_o) => {},
        Err(e) => return Err(e),
    };

    return Ok(());
}

// Set the 'current' generation to the latest generation.
pub fn latest() -> Result<(), io::Error> {
    match set_current(match latest_number() {
        Ok(o) => o,
        Err(e) => return Err(e),
    }) {
        Ok(_o) => {},
        Err(e) => return Err(e),
    };

    return Ok(());
}

// Set the 'current' generation to a specific generation.
pub fn set_current(to: usize) -> Result<(), io::Error> {
    if to > match latest_number() {
        Ok(o) => o,
        Err(e) => return Err(e),
    } {
        error!("Out of range! Too high!");
        return Err(custom_error("Out of range!"));
    }

    if to < 1 {
        error!("Out of range! Too low!");
        return Err(custom_error("Out of range!"));
    }

    match write_file(to.to_string().trim(), format!("{}/current", places::gens()).as_str()) {
        Ok(_o) => {
            info!("Set 'current' to: {}", to);
            return Ok(());
        },
        Err(e) => {
            error!("Failed to create/write 'current' tracking file!");
            return Err(e);
        },
    };
}

// Get the 'current' generation number.
pub fn get_current() -> Result<usize, io::Error> {
    let contents = match read_file(format!("{}/current", places::gens()).as_str()) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to read 'current' file!");
            return Err(e);
        },
    };

    let generation: usize = match contents.trim().parse() {
        Ok(o) => o,
        Err(_e) => {
            error!("Failed to parse number from 'current' file! (Maybe 'current' file is corrupted?)");
            return Err(custom_error("Failed to parse number out of 'current' file!"));
        },
    };

    return Ok(generation);
}

// List all generations. (NORMAL)
pub fn list() -> Result<Vec<(String, String, bool)>, io::Error> {
    return list_core(true);
}

// List all generations. (ISOLATED MODE | For avoiding errors with called un-needed functions!)
pub fn list_with_no_calls() -> Result<Vec<(String, String, bool)>, io::Error> {
    return list_core(false);
}

// List all generations. (CORE)
fn list_core(calls: bool) -> Result<Vec<(String, String, bool)>, io::Error> {
    let gen_listed = match list_directory(places::gens().as_str()) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to list the generations directory! ({})", places::gens());
            return Err(e);
        },
    };

    let mut generations: Vec<String> = Vec::new();

    for i in gen_listed.iter() {
        match path_type(i) {
            PathType::File => {},
            PathType::Directory => generations.push(i.to_string()),
            PathType::Invalid => {
                error!("Found invalid path! (Listing generations.)");
                return Err(custom_error("Found invalid path."));
            },
        };
    }

    let mut gens_with_info: Vec<(String, String, bool)> = Vec::new();

    for i in generations.iter() {
        let generation_name = name_from_path(i);
        let commit_msg = match read_file(format!("{}/commit", i).as_str()) {
            Ok(o) => o,
            Err(_e) => "<< COMMIT MESSAGE MISSING >>".to_string(),
        };

        let current_number: usize;

        if calls == true {
            current_number = match get_current() {
                Ok(o) => o,
                Err(e) => return Err(e),
            };
        } else {
            current_number = 0;
        }

        let is_current: bool;

        if generation_name == current_number.to_string() {
            is_current = true;
        } else {
            is_current = false;
        }

        gens_with_info.push((generation_name, commit_msg, is_current));
    }

    return Ok(gens_with_info);
}

// Print out the list of generations.
pub fn list_print() -> Result<(), io::Error> {
    let list_items = match list() {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let list_items_sorted = match sort_list_vector(&list_items) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let mut max_digits: usize = 0;

    if list_items_sorted.len() > 0 {
        max_digits = list_items_sorted[list_items_sorted.len() - 1].0.to_string().trim().len();
    }

    for i in list_items_sorted.iter() {
        let misc_text: String;

        if i.2 {
            misc_text = format!(" {}{}{}", "[".bright_black().bold(), "CURRENT".bright_green().bold(), "]".bright_black().bold());
        } else {
            misc_text = "".to_string();
        }

        let mut tabbed = String::new();

        for _j in 0..(max_digits - i.0.trim().len()) {
            tabbed.push_str(" ");
        }

        generic!("{}{} ... ({}){}", tabbed, i.0, i.1, misc_text);
    }

    return Ok(());
}

// Get only list vector generation names.
fn get_list_vector_names(list_vec: &Vec<(String, String, bool)>) -> Vec<String> {
    let mut new_vec: Vec<String> = Vec::new();

    for i in list_vec.iter() {
        new_vec.push(i.0.to_string());
    }

    return new_vec;
}

// Sort list vector.
fn sort_list_vector(list_vec: &Vec<(String, String, bool)>) -> Result<Vec<(String, String, bool)>, io::Error> {
    if list_vec.len() == 0 {
        return Ok(list_vec.clone());
    }

    let list_vec_names = get_list_vector_names(&list_vec);

    let mut min_spot: usize = 0;

    while list_vec_names.contains(&min_spot.to_string()) == false {
        min_spot += 1;
    }

    let mut new_vec: Vec<(String, String, bool)> = Vec::new();

    while new_vec.len() != list_vec.len() {
        for i in list_vec.iter() {
            let i_num: usize = match i.0.trim().parse() {
                Ok(o) => o,
                Err(_e) => {
                    error!("Failed to parse invalid generation name! ({})", i.0);
                    return Err(custom_error("Failed to parse invalid generation name!"));
                },
            };

            if i_num == new_vec.len() + min_spot {
                new_vec.push(i.clone());
            }
        }
    }

    return Ok(new_vec);
}

// Get the 'current' generation TOML file.
pub fn current_gen() -> String {
    let current = read_file(format!("{}/current", places::gens()).as_str()).unwrap();
    let current = current.trim();

    return format!("{}/{}/gen.toml", places::gens(), current);
}
