// Source files to include.
mod library;
mod convert;
mod filesystem;
mod places;
mod cli;
mod log;
mod generation;
mod repeated;
mod config;
mod dir;
mod utils;
mod obj_print;

// Import stuff from source files and crates.
use clap::Parser;
use log::*;
use std::io;
use filesystem::*;
use library::*;
use config::ConfigSide;

// The exit code for the program.
#[derive(PartialEq)]
enum ExitCode {
    Success,
    Fail,
}

// Use this function for testing code!
fn test_code() {
}

// We are using main() to run another function, and exit according to the exit code.
fn main() {
    if app() == ExitCode::Fail {
        std::process::exit(1);
    }
}

// The "main" function.
fn app() -> ExitCode {
    test_code(); // This function is for nothing but testing code whilst developing!

    match is_root_user() {
        true => {
            error!("Cannot run as root! Please run as the normal user!");
            return ExitCode::Fail;
        },

        false => {},
    };

    let args = cli::Cli::parse();

    match &args.command {
        cli::Commands::Setup => {},
        _ => {
            if path_exists(places::base().as_str()) == false {
                error!("It seems that the program is not set up!");
                return ExitCode::Fail;
            }
        },
    }

    #[allow(unreachable_patterns)]
    match &args.command {
        cli::Commands::Commit(c) => {
            info!("Committing user generation...");

            match generation::commit(c.msg.as_str()) {
                Ok(_o) => info!("Committed generation successfully! (\"{}\")", c.msg),
                Err(_e) => return ExitCode::Fail,
            };
        },
        cli::Commands::Build => {
            info!("Building 'current' generation...");

            match generation::build() {
                Ok(_o) => info!("Built generation successfully!"),
                Err(_e) => return ExitCode::Fail,
            };
        },
        cli::Commands::Rollback(r) => {
            info!("Rolling back by {} generations...", r.by);

            match generation::rollback(r.by) {
                Ok(_o) => info!("Rolled back successfully!"),
                Err(_e) => return ExitCode::Fail,
            };
        },
        cli::Commands::Latest => {
            info!("Jumping to latest generation...");

            match generation::latest() {
                Ok(_o) => info!("Jumped to latest successfully!"),
                Err(_e) => return ExitCode::Fail,
            };
        },
        cli::Commands::SetCurrent(s) => {
            info!("Jumping to generation {}...", s.to);

            match generation::set_current(s.to) {
                Ok(_o) => info!("Jumped to generation {} successfully!", s.to),
                Err(_e) => return ExitCode::Fail,
            };
        },
        cli::Commands::List => {
            match generation::list_print() {
                Ok(_o) => {},
                Err(_e) => return ExitCode::Fail,
            };
        },
        cli::Commands::Setup => {
            info!("Beginning setup...");

            match setup() {
                Ok(_o) => info!("Set up the program successfully!"),
                Err(_e) => return ExitCode::Fail,
            };
        },
        cli::Commands::GenInfo => {
            let generation = match config::gen(ConfigSide::User) {
                Ok(o) => o,
                Err(_e) => return ExitCode::Fail,
            };

            obj_print::generation(&generation);
        },
        cli::Commands::InitConfig { force } => {
            info!("Creating user configuration...");

            match config::init_user_config(*force) {
                Ok(_o) => info!("Created user configuration successfully!"),
                Err(_e) => return ExitCode::Fail,
            };
        },
        cli::Commands::WhatIsLatest => {
            info!("Latest generation number is: {}", match generation::latest_number() {
                Ok(o) => o,
                Err(_e) => return ExitCode::Fail,
            });
        },
        cli::Commands::DeleteOld(h) => {
            info!("Deleting old generations...");

            match generation::delete_old(h.how_many) {
                Ok(_o) => info!("Successfully deleted {} generations!", h.how_many),
                Err(_e) => return ExitCode::Fail,
            };
        },
        _ => {
            error!("Command not usable yet!");
            return ExitCode::Fail;
        },
    };

    return ExitCode::Success;
}

// Function that sets up the program.
fn setup() -> Result<(), io::Error> {
    match places::setup() {
        Ok(_o) => info!("Core directories verified successfully!"),
        Err(e) => return Err(e),
    };

    return Ok(());
}
