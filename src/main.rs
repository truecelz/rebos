// Source files to include.
mod library; // Full of functions.
mod convert; // Convert one type to another type.
mod filesystem; // Functions for interacting with the filesystem.
mod places; // Where is stuff stored?
mod cli; // For argument parsing and command structuring.
mod log; // Contains macros and stuff for pretty output messages in the terminal!
mod generation; // The generations system.
mod repeated; // Used for repeated code.
mod config; // Configuration stuff.
mod dir; // Interfacing with the dirs crate.
mod obj_print; // Print objects.
mod package_management; // Stuff related to package management.
mod system; // Used for getting system information.
mod pkg_managers; // Package managers.

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
        cli::Commands::Gen { command } => {
            match command {
                cli::GenCommands::Commit(c) => {
                    info!("Committing user generation...");

                    match generation::commit(c.msg.as_str()) {
                        Ok(_o) => info!("Committed generation successfully! (\"{}\")", c.msg),
                        Err(_e) => return ExitCode::Fail,
                    };
                },
                cli::GenCommands::List => {
                    match generation::list_print() {
                        Ok(_o) => {},
                        Err(_e) => return ExitCode::Fail,
                    };
                },
                cli::GenCommands::Info => {
                    let generation = match generation::gen(ConfigSide::User) {
                        Ok(o) => o,
                        Err(_e) => return ExitCode::Fail,
                    };

                    obj_print::generation(&generation);
                },
                cli::GenCommands::Latest => {
                    info!("Latest generation number is: {}", match generation::latest_number() {
                        Ok(o) => o,
                        Err(_e) => return ExitCode::Fail,
                    });
                },
                cli::GenCommands::DeleteOld(h) => {
                    info!("Deleting old generations...");

                    match generation::delete_old(h.how_many) {
                        Ok(_o) => info!("Successfully deleted {} generations!", h.how_many),
                        Err(_e) => return ExitCode::Fail,
                    };
                },
                cli::GenCommands::Delete(g) => {
                    match generation::delete(g.generation) {
                        Ok(_o) => {}, // Handled by delete().
                        Err(_e) => return ExitCode::Fail,
                    };
                },
                cli::GenCommands::Current { command } => {
                    match command {
                        cli::CurrentCommands::Build => {
                            info!("Building 'current' generation...");

                            match generation::build() {
                                Ok(_o) => info!("Built generation successfully!"),
                                Err(_e) => return ExitCode::Fail,
                            };
                        },
                        cli::CurrentCommands::Rollback(r) => {
                            info!("Rolling back by {} generations...", r.by);

                            match generation::rollback(r.by) {
                                Ok(_o) => info!("Rolled back successfully!"),
                                Err(_e) => return ExitCode::Fail,
                            };
                        },
                        cli::CurrentCommands::ToLatest => {
                            info!("Jumping to latest generation...");

                            match generation::latest() {
                                Ok(_o) => info!("Jumped to latest successfully!"),
                                Err(_e) => return ExitCode::Fail,
                            };
                        },
                        cli::CurrentCommands::Set(s) => {
                            info!("Jumping to generation {}...", s.to);

                            match generation::set_current(s.to) {
                                Ok(_o) => info!("Jumped to generation {} successfully!", s.to),
                                Err(_e) => return ExitCode::Fail,
                            };
                        },
                        _ => {
                            error!("Command not usable yet!");
                            return ExitCode::Fail;
                        },
                    };
                },
                _ => {
                    error!("Command not usable yet!");
                    return ExitCode::Fail;
                },
            };
        },
        cli::Commands::Setup => {
            info!("Beginning setup...");

            match setup() {
                Ok(_o) => info!("Set up the program successfully!"),
                Err(_e) => return ExitCode::Fail,
            };
        },
        cli::Commands::InitConfig => {
            info!("Creating user configuration...");

            match config::init_user_config() {
                Ok(_o) => info!("Created user configuration successfully!"),
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
