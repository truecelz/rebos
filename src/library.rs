#![allow(dead_code)]

use colored::Colorize;
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
