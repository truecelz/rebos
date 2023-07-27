#![allow(dead_code)]

use crate::convert::str_to_string_vec;

pub fn cut(full: &str, fword: u32, dword: char) -> String {
    let vecced = str_to_string_vec(full, dword.to_string().as_str());

    let fspot = fword - 1;

    if vecced.len() < fword as usize {
        return String::from("");
    }

    return vecced[fspot as usize].to_string();
}
