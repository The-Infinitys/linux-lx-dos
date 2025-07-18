use clap::Parser;
use clap::Subcommand;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Suppress all output
    #[arg(short, long, conflicts_with_all = &["verbose","debug"])]
    pub quiet: bool,

    /// Increase message verbosity
    #[arg(short, long, conflicts_with_all = &["quiet", "debug"])]
    pub verbose: bool,

    /// Enable debug output
    #[arg(short, long, conflicts_with_all = &["verbose", "quiet"])]
    pub debug: bool,

    /// Run in command-line interface mode
    #[arg(long, conflicts_with = "gui")]
    pub cli: bool,

    /// Run in graphical user interface mode
    #[arg(long, default_value_t = true, conflicts_with = "cli")]
    pub gui: bool,
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start Lx-DOS
    Start,
    /// Stop Lx-DOS
    Stop,
    /// Show welcome message
    Welcome,
    /// run Lx-DOS
    Run,
}
