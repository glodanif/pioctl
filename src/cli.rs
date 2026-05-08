pub mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(long, help = "Print commands without executing them")]
    pub dry_run: bool,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "List all connected monitors and their current configuration")]
    Monitors,
    #[command(about = "List all available audio output sinks")]
    AudioSinks,
    #[command(about = "List all profiles defined in the config directory")]
    Profiles,
    #[command(about = "Show the currently active profile")]
    Current,
    #[command(about = "Re-apply the current profile, with an optional delay before starting")]
    Restore { delay_ms: Option<u64> },
    #[command(about = "Apply a specific profile by its ID (config filename stem)")]
    Apply { profile_id: String },
    #[command(about = "Apply the next profile in alphabetical order, cycling back to the first")]
    ApplyNext,
}
