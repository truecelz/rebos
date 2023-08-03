#![allow(dead_code)]

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
/// Make any Linux distribution reproducable!
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a generation system command.
    Gen {
        #[command(subcommand)]
        command: GenCommands,
    },
    /// Run the program setup.
    Setup,
    /// Create a default Dister configuration.
    InitConfig {
        /// Forces the command to create the default config, even if it means overwriting.
        #[clap(long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum GenCommands {
    /// Confirm your custom generation, and make it the 'current' generation.
    Commit(Commit),
    /// List all system generations.
    List,
    /// Get information on the generation in the user's config.
    Info,
    /// Print out what the latest system generation number is.
    Latest,
    /// Delete older generations.
    DeleteOld(DeleteOld),
    /// Command related to the 'current' generation.
    Current {
        #[command(subcommand)]
        command: CurrentCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum CurrentCommands {
    /// Build the 'current' generation. (You can always roll back later.)
    Build,
    /// Rollback to a previous generation. (You still need to build after rolling back.)
    Rollback(Rollback),
    /// Set the 'current' generation to the latest generation.
    ToLatest,
    /// Set the 'current' generation to a specific generation.
    Set(SetCurrent),
}

#[derive(Parser, Debug)]
pub struct DeleteOld {
    /// Starting at the oldest generation, how many should be deleted?
    pub how_many: usize,
}

#[derive(Parser, Debug)]
pub struct Commit {
    /// The commit message shows up in the list command.
    pub msg: String,
}

#[derive(Parser, Debug)]
pub struct SetCurrent {
    /// Generation to jump to.
    pub to: usize,
}

#[derive(Parser, Debug)]
pub struct Rollback {
    /// How many generations to rollback by.
    pub by: isize,
}
