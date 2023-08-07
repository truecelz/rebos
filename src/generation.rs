#![allow(dead_code)]

use std::io;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use crate::convert::*;
use crate::filesystem::*;
use crate::places;
use crate::log::*;
use crate::library::*;
use crate::package_management::*;
use crate::{info, warning, error, generic, note};
use crate::config::{Config, ConfigSide};
use crate::config::config_for;
use crate::system;
use crate::flatpak;

// The structure for a generation.
#[derive(PartialEq, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct Generation {
    pub pkgs: Vec<String>,
    pub flatpaks: Vec<String>,
}

impl Default for Generation {
    fn default() -> Generation {
        return Generation {
            pkgs: Vec::new(),
            flatpaks: Vec::new(),
        };
    }
}

impl GenerationUtils for Generation {
    fn extend(&mut self, other_gen: Generation) {
        self.pkgs.extend(other_gen.pkgs);
        self.flatpaks.extend(other_gen.flatpaks);
    }
}

pub trait GenerationUtils {
    // Extend all of the fields from one Generation object to another, another being the caller.
    fn extend(&mut self, other_gen: Generation);
}

// Return generation structure for...
pub fn gen(side: ConfigSide) -> Result<Generation, io::Error> {
    let mut generation = match read_to_gen(config_for(Config::Generation, side).as_str()) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let system_hostname = match system::hostname() {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    if side == ConfigSide::User {
        generation.extend(match read_to_gen(format!("{}/machines/{}/gen.toml", places::base_user(), system_hostname).as_str()) {
            Ok(o) => o,
            Err(e) => return Err(e),
        });
    }

    return Ok(generation);
}

// Read a file and return a Generation object.
fn read_to_gen(path: &str) -> Result<Generation, io::Error> {
    return Ok(match toml::from_str(match read_file(path) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to read generation TOML file!");
            return Err(e);
        },
    }.as_str()) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to parse generation TOML file!");
            error!("TOML Error: {:?}", e);

            return Err(custom_error("Failed to parse TOML file!"));
        },
    });
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

    let user_gen = match gen(ConfigSide::User) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let user_gen_string = match toml::to_string(&user_gen) {
        Ok(o) => o,
        Err(_e) => {
            error!("Failed to convert user generation to string!");
            return Err(custom_error("Failed to convert user generation to string!"));
        },
    };

    let files = vec![
        (msg, format!("{}/commit", gen_dir)),
        (user_gen_string.as_str(), format!("{}/gen.toml", gen_dir)),
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

    let current_num = match get_current() {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let curr_gen = match gen(ConfigSide::System) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    match read_file(format!("{}/built", places::gens()).as_str()) {
        Ok(o) => {
            let built_gen = match read_to_gen(format!("{}/{}/gen.toml", places::gens(), o.trim()).as_str()) {
                Ok(o) => o,
                Err(e) => return Err(e),
            };

            let built_pkgs_string = string_vec_to_string(&built_gen.pkgs, "\n");
            let curr_pkgs_string = string_vec_to_string(&curr_gen.pkgs, "\n");

            let built_flatpaks_string = string_vec_to_string(&built_gen.flatpaks, "\n");
            let curr_flatpaks_string = string_vec_to_string(&curr_gen.flatpaks, "\n");

            let pkgs_diffs = history(&built_pkgs_string, &curr_pkgs_string);
            let flatpaks_diffs = history(&built_flatpaks_string, &curr_flatpaks_string);

            let mut pkgs_to_install: Vec<String> = Vec::new();
            let mut pkgs_to_remove: Vec<String> = Vec::new();

            let mut flatpaks_to_install: Vec<String> = Vec::new();
            let mut flatpaks_to_remove: Vec<String> = Vec::new();

            for i in pkgs_diffs.iter() {
                match i.mode {
                    HistoryMode::Add => pkgs_to_install.push(i.line.to_string()),
                    HistoryMode::Remove => pkgs_to_remove.push(i.line.to_string()),
                };
            }

            for i in flatpaks_diffs.iter() {
                match i.mode {
                    HistoryMode::Add => flatpaks_to_install.push(i.line.to_string()),
                    HistoryMode::Remove => flatpaks_to_remove.push(i.line.to_string()),
                };
            }

            match install(&pkgs_to_install) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };

            match uninstall(&pkgs_to_remove) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };

            match flatpak::install(&flatpaks_to_install) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };

            match flatpak::uninstall(&flatpaks_to_remove) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };

            println!("");

            info!("#################");
            info!("#    SUMMARY    #");
            info!("#################");

            println!("");

            info!("Packages:");
            print_history(&pkgs_diffs);

            info!("Flatpaks:");
            print_history(&flatpaks_diffs);
        },
        Err(_e) => {
            match install(&curr_gen.pkgs) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };

            match flatpak::install(&curr_gen.flatpaks) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };

            note!("This output will look slightly different than when you run this command again, because this is your first time building a generation on this system.");
        },
    };

    match set_built(current_num) {
        Ok(_o) => {},
        Err(e) => return Err(e),
    };

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

// Set the 'built' generation to a specific generation.
pub fn set_built(to: usize) -> Result<(), io::Error> {
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

    match write_file(to.to_string().trim(), format!("{}/built", places::gens()).as_str()) {
        Ok(_o) => {
            info!("Set 'built' to: {}", to);
            return Ok(());
        },
        Err(e) => {
            error!("Failed to create/write 'built' tracking file!");
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

// Get the currently built generation number.
pub fn get_built() -> Result<usize, io::Error> {
    let contents = match read_file(format!("{}/built", places::gens()).as_str()) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to read 'built' file!");
            return Err(e);
        },
    };

    let generation: usize = match contents.trim().parse() {
        Ok(o) => o,
        Err(_e) => {
            error!("Failed to parse number from 'built' file! (Maybe 'built' file is corrupted?)");
            return Err(custom_error("Failed to parse number out of 'built' file!"));
        },
    };

    return Ok(generation);
}

// Has a generation been built yet?
pub fn been_built() -> bool {
    return path_exists(format!("{}/built", places::gens()).as_str());
}

// Delete old generations.
pub fn delete_old(how_many: usize) -> Result<(), io::Error> {
    let offset = match get_oldest() {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    for i in offset..(how_many + offset) {
        match delete(i) {
            Ok(_o) => {}, // This is a rare instance where the matched function actually did the info!() itself!
            Err(e) => return Err(e),
        };
    }

    return Ok(());
}

// Delete a specific generation.
pub fn delete(generation: usize) -> Result<(), io::Error> {
    if match is_current(generation) {
        Ok(o) => o,
        Err(e) => return Err(e),
    } {
        warning!("Could not delete generation {}, because it is the 'current' generation, and is protected!", generation);
        return Ok(());
    }

    if been_built() {
        if match is_built(generation) {
            Ok(o) => o,
            Err(e) => return Err(e),
        } {
            warning!("Could not delete generation {}, because it is the currently built generation, and is protected!", generation);
            return Ok(());
        }
    }

    match remove_directory(format!("{}/{}", places::gens(), generation).as_str()) {
        Ok(_o) => info!("Deleted generation: {}", generation),
        Err(e) => {
            error!("Failed to delete generation: {}", generation);
            return Err(e);
        },
    };

    return Ok(());
}

// Return true or false based on if the given generation is the 'current' generation.
pub fn is_current(generation: usize) -> Result<bool, io::Error> {
    if generation == match get_current() {
        Ok(o) => o,
        Err(e) => return Err(e),
    } {
        return Ok(true);
    }

    else {
        return Ok(false);
    }
}

// Return true or false based on if the given generation is the built generation.
pub fn is_built(generation: usize) -> Result<bool, io::Error> {
    if generation == match get_built() {
        Ok(o) => o,
        Err(e) => return Err(e),
    } {
        return Ok(true);
    }

    else {
        return Ok(false);
    }
}

// List all generations. (NORMAL)
pub fn list() -> Result<Vec<(String, String, bool, bool)>, io::Error> {
    return list_core(true);
}

// List all generations. (ISOLATED MODE | For avoiding errors with called un-needed functions!)
pub fn list_with_no_calls() -> Result<Vec<(String, String, bool, bool)>, io::Error> {
    return list_core(false);
}

// List all generations. (CORE)
fn list_core(calls: bool) -> Result<Vec<(String, String, bool, bool)>, io::Error> {
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

    let mut gens_with_info: Vec<(String, String, bool, bool)> = Vec::new();

    for i in generations.iter() {
        let generation_name = name_from_path(i);
        let commit_msg = match read_file(format!("{}/commit", i).as_str()) {
            Ok(o) => o,
            Err(_e) => "<< COMMIT MESSAGE MISSING >>".to_string(),
        };

        let current_number: usize;
        let built_number: usize;

        if calls == true {
            current_number = match get_current() {
                Ok(o) => o,
                Err(e) => return Err(e),
            };
            built_number = match get_built() {
                Ok(o) => o,
                Err(_e) => 0,
            };
        } else {
            current_number = 0;
            built_number = 0;
        }

        let is_current: bool;
        let is_built: bool;

        if generation_name == current_number.to_string() {
            is_current = true;
        } else {
            is_current = false;
        }

        if generation_name == built_number.to_string() {
            is_built = true;
        } else {
            is_built = false;
        }

        gens_with_info.push((generation_name, commit_msg, is_current, is_built));
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
        let mut misc_text = String::new();

        if i.2 {
            misc_text.push_str(format!(" {}{}{}", "[".bright_black().bold(), "CURRENT".bright_green().bold(), "]".bright_black().bold()).as_str());
        }

        if i.3 {
            misc_text.push_str(format!(" {}{}{}", "[".bright_black().bold(), "BUILT".bright_yellow().bold(), "]".bright_black().bold()).as_str());
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
fn get_list_vector_names(list_vec: &Vec<(String, String, bool, bool)>) -> Vec<String> {
    let mut new_vec: Vec<String> = Vec::new();

    for i in list_vec.iter() {
        new_vec.push(i.0.to_string());
    }

    return new_vec;
}

// Sort list vector.
fn sort_list_vector(list_vec: &Vec<(String, String, bool, bool)>) -> Result<Vec<(String, String, bool, bool)>, io::Error> {
    if list_vec.len() == 0 {
        return Ok(list_vec.clone());
    }

    let list_vec_names = get_list_vector_names(&list_vec);

    let mut list_vec_nums: Vec<usize> = Vec::new();

    for i in list_vec_names.iter() {
        list_vec_nums.push(match i.trim().parse() {
            Ok(o) => o,
            Err(_e) => {
                error!("Failed to parse invalid generation name! ({})", i.trim());
                return Err(custom_error("Failed to parse invalid generation name!"));
            },
        });
    }

    list_vec_nums.sort();

    let mut new_vec: Vec<(String, String, bool, bool)> = Vec::new();

    for i in list_vec_nums.iter() {
        for j in list_vec.iter() {
            let j_num: usize = match j.0.trim().parse() {
                Ok(o) => o,
                Err(_e) => {
                    error!("Failed to parse invalid generation name! ({})", j.0.trim());
                    return Err(custom_error("Failed to parse invalid generation name!"));
                },
            };

            if &j_num == i {
                new_vec.push(j.clone());
                break;
            }
        }
    }

    return Ok(new_vec);
}

// Get oldest generation name.
pub fn get_oldest() -> Result<usize, io::Error> {
    let gen_names = get_list_vector_names(&match sort_list_vector(&match list_with_no_calls() {
        Ok(o) => o,
        Err(e) => return Err(e),
    }) {
        Ok(o) => o,
        Err(e) => return Err(e),
    });

    if gen_names.len() == 0 {
        error!("Tried to call generation::get_oldest(), when there are no generations!");
        return Err(custom_error("Not enough generations!"));
    }

    let oldest_name = gen_names[0].to_string();

    let oldest_number: usize = match oldest_name.trim().parse() {
        Ok(o) => o,
        Err(_e) => {
            error!("Failed to parse invalid generation name! ({})", oldest_name.trim());
            return Err(custom_error("Failed to parse invalid generation name!"));
        },
    };

    return Ok(oldest_number);
}

// Get the 'current' generation TOML file.
pub fn current_gen() -> Result<String, io::Error> {
    let current = match get_current() {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    return Ok(format!("{}/{}/gen.toml", places::gens(), current));
}
