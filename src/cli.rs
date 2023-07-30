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
    /// Confirm your custom generation, and make it the 'current' generation.
    Commit(Commit),
    /// Build the 'current' generation. (You can always roll back later.)
    Build,
    /// Rollback to a previous generation. (You still need to build after rolling back.)
    Rollback(Rollback),
    /// Set the 'current' generation to the latest generation.
    Latest,
    /// Set the 'current' generation to a specific generation.
    SetCurrent(SetCurrent),
    /// List all system generations.
    List,
    /// Run the program setup.
    Setup,
    /// Get information on the generation in the user's config.
    GenInfo,
    /// Create a default Dister configuration.
    InitConfig {
        /// Forces the command to create the default config, even if it means overwriting.
        #[clap(long)]
        force: bool,
    },
}

#[derive(Parser, Debug)]
pub struct Commit {
    pub msg: String,
}

#[derive(Parser, Debug)]
pub struct SetCurrent {
    pub to: usize,
}

#[derive(Parser, Debug)]
pub struct Rollback {
    /// How many generations to rollback by.
    pub by: isize,
}
