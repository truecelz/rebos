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
    /// Confirm changes and build! (You can always roll back later.)
    Build,
    /// Rollback to a previous generation.
    Rollback(Rollback),
}

#[derive(Parser, Debug)]
pub struct Rollback {
    /// How many generations to rollback by.
    pub by: usize,
}
