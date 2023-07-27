#![allow(dead_code)]

pub fn home() -> String {
    return dirs::home_dir().unwrap().display().to_string();
}

pub fn config() -> String {
    return dirs::config_dir().unwrap().display().to_string();
}
