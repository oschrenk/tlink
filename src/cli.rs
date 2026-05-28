use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tlink", about = "tmux:// deeplink CLI for macOS")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Interactive TUI wizard: select terminal, register tmux:// URI scheme
    Setup,
    /// Handle a tmux:// URI (invoked by the OS when a deeplink is clicked)
    Open {
        /// The tmux:// URI, e.g. tmux://mysession/0/1
        uri: String,
    },
    /// Show URI scheme registration status and active tmux sessions
    Status,
    /// Re-register the URI scheme handler without re-running setup
    Restart,
    /// Run diagnostic checks and report pass/fail
    Doctor,
}
