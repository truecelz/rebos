// Source files to include.
mod library; // Full of functions.
mod convert; // Convert one type to another type.
mod places; // Where is stuff stored?
mod cli; // For argument parsing and command structuring.
mod generation; // The generations system.
mod config; // Configuration stuff.
mod dir; // Interfacing with the dirs crate.
mod obj_print; // Print objects.
mod obj_print_boilerplate; // Boilerplate code for obj print.
mod package_management; // Stuff related to package management.
mod system; // Used for getting system information.
mod hook; // Hook stuff.

// Import stuff from source files and crates.
use clap::Parser;
use std::io;
use library::*;
use config::ConfigSide;
use colored::Colorize;
use piglog::prelude::*;
use piglog::*;

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
        cli::Commands::Setup => (),
        _ => {
            if places::base().exists() == false {
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
                        Ok(_) => success!("Committed generation successfully! (\"{}\")", c.msg),
                        Err(_) => return ExitCode::Fail,
                    };
                },
                cli::GenCommands::List => {
                    match generation::list_print() {
                        Ok(_) => (),
                        Err(_) => return ExitCode::Fail,
                    };
                },
                cli::GenCommands::CleanDups => {
                    match generation::management::clean_dups(true) {
                        Ok(o) => success!("Deleted {o} generations!"),
                        Err(_) => return ExitCode::Fail,
                    };
                },
                cli::GenCommands::Align => {
                    match generation::management::align(true) {
                        Ok(o) => success!("Aligned {o} generations!"),
                        Err(_) => return ExitCode::Fail,
                    };
                },
                cli::GenCommands::TidyUp => {
                    match generation::management::tidy_up() {
                        Ok(_) => (),
                        Err(_) => return ExitCode::Fail,
                    };
                },
                cli::GenCommands::Info => {
                    let generation = match generation::gen(ConfigSide::User) {
                        Ok(o) => o,
                        Err(_) => return ExitCode::Fail,
                    };

                    obj_print::generation(&generation);
                },
                cli::GenCommands::Latest => {
                    info!("Latest generation number is: {}", match generation::latest_number() {
                        Ok(o) => o,
                        Err(_) => return ExitCode::Fail,
                    });
                },
                cli::GenCommands::DeleteOld(h) => {
                    info!("Deleting old generations...");

                    match generation::delete_old(h.how_many, true) {
                        Ok(_) => success!("Successfully deleted {} generations!", h.how_many),
                        Err(_) => return ExitCode::Fail,
                    };
                },
                cli::GenCommands::Delete(g) => {
                    match generation::delete(g.generation, true) {
                        Ok(_) => (), // Handled by delete().
                        Err(_) => return ExitCode::Fail,
                    };
                },
                cli::GenCommands::Diff { old, new } => {
                    if generation::gen_exists(*old) == false || generation::gen_exists(*new) == false {
                        fatal!("Generation not found!");

                        return ExitCode::Fail;
                    }

                    let gen_1 = generation::get_gen_from_usize(*old).unwrap();
                    let gen_2 = generation::get_gen_from_usize(*new).unwrap();

                    let commit_1 = generation::get_gen_commit_from_usize(*old).unwrap();
                    let commit_2 = generation::get_gen_commit_from_usize(*new).unwrap();

                    let history = library::history_gen(&gen_1, &gen_2);

                    println!(
                        "\n{} {} {}",
                        commit_1.bright_cyan().bold(),
                        "->".bright_black().bold(),
                        commit_2.bright_cyan().bold()
                    );

                    println!("");

                    library::print_history_gen(&history);
                },
                cli::GenCommands::Current { command } => {
                    match command {
                        cli::CurrentCommands::Build => {
                            info!("Building 'current' generation...");

                            match generation::build() {
                                Ok(_) => success!("Built generation successfully!"),
                                Err(_) => return ExitCode::Fail,
                            };
                        },
                        cli::CurrentCommands::Rollback(r) => {
                            info!("Rolling back by {} generations...", r.by);

                            match generation::rollback(r.by, true) {
                                Ok(_) => success!("Rolled back successfully!"),
                                Err(_) => return ExitCode::Fail,
                            };
                        },
                        cli::CurrentCommands::ToLatest => {
                            info!("Jumping to latest generation...");

                            match generation::latest(true) {
                                Ok(_) => success!("Jumped to latest successfully!"),
                                Err(_) => return ExitCode::Fail,
                            };
                        },
                        cli::CurrentCommands::Set(s) => {
                            info!("Jumping to generation {}...", s.to);

                            match generation::set_current(s.to, true) {
                                Ok(_) => success!("Jumped to generation {} successfully!", s.to),
                                Err(_) => return ExitCode::Fail,
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
                Ok(_) => success!("Set up the program successfully!"),
                Err(_) => return ExitCode::Fail,
            };
        },
        cli::Commands::InitConfig => {
            info!("Creating user configuration...");

            match config::init_user_config() {
                Ok(_) => success!("Created user configuration successfully!"),
                Err(_) => return ExitCode::Fail,
            };
        },
        cli::Commands::API { command } => {
            match command {
                cli::APICommands::Echo { log_mode, message } => {
                    piglog::log_core_print(message.to_string(), *log_mode);
                },
                cli::APICommands::EchoGeneric { message } => {
                    piglog::log_generic_print(message.to_string());
                },
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
        Ok(_) => success!("Core directories verified successfully!"),
        Err(e) => return Err(e),
    };

    return Ok(());
}
