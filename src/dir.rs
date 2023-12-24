#![allow(dead_code)]

use fspp::Path;

macro_rules! dirs_path {
    ($dir: ident) => {
        Path::new(&dirs::$dir().unwrap().display().to_string())
    };
}

pub fn home() -> Path {
    return dirs_path!(home_dir);
}

pub fn config() -> Path {
    return dirs_path!(config_dir);
}
