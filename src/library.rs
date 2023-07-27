#![allow(dead_code)]

use std::io;
use colored::Colorize;
use users::get_current_username;
use crate::convert::*;

#[derive(PartialEq)]
pub enum HistoryMode {
    Remove,
    Add,
}

pub struct History {
    pub mode: HistoryMode,
    pub line: String,
}

pub fn name_from_path(path: &str) -> String {
    let converted = str_to_string_vec(path, "/");

    return converted[converted.len() - 1].to_string();
}

pub fn custom_error(error: &str) -> io::Error {
    return io::Error::new(io::ErrorKind::Other, error);
}

pub fn is_root_user() -> bool {
    if username() == "root" {
        return true;
    } else {
        return false;
    }
}

pub fn username() -> String {
    let username: String = match get_current_username() {
        Some(uname) => uname.to_str().unwrap().to_string(),
        None => panic!("Unable to get username!"),
    };

    return username;
}

pub fn history(str_1: &str, str_2: &str) -> Vec<History> {
    let lines_1 = str_to_string_vec(str_1, "\n");
    let lines_2 = str_to_string_vec(str_2, "\n");

    let mut history_vec: Vec<History> = Vec::new();

    for i in lines_1.iter() {
        if lines_2.contains(i) == false {
            history_vec.push(History {
                mode: HistoryMode::Remove,
                line: i.to_string(),
            });
        }
    }

    for i in lines_2.iter() {
        if lines_1.contains(i) == false {
            history_vec.push(History {
                mode: HistoryMode::Add,
                line: i.to_string(),
            });
        }
    }

    return history_vec;
}

pub fn print_history(diff_vec: &Vec<History>) {
    for i in diff_vec.iter() {
        if i.mode == HistoryMode::Remove {
            println!("{}", format!("- {}", i.line).red().bold());
        }

        else if i.mode == HistoryMode::Add {
            println!("{}", format!("+ {}", i.line).green().bold());
        }

        else {
            println!("{}", format!("  {}", i.line));
        }
    }
}
