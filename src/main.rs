// Source files to include.
mod library;
mod convert;
mod filesystem;
mod places;
mod cli;
mod log;
mod generation;

// Import stuff from source files and crates.
use clap::Parser;
use log::*;
use std::io;

// The exit code for the program.
#[derive(PartialEq)]
enum ExitCode {
    Success,
    Fail,
}

// We are using main() to run another function, and exit according to the exit code.
fn main() {
    if app() == ExitCode::Fail {
        std::process::exit(1);
    }
}

// The "main" function.
fn app() -> ExitCode {
    match setup() {
        Ok(_o) => {},
        Err(_e) => return ExitCode::Fail,
    };

    let args = cli::Cli::parse();

    #[allow(unreachable_patterns)]
    match &args.command {
        cli::Commands::Build => {
            info!("Building generation...");
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
            }
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
    return Ok(());
}
