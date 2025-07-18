use clap::Parser;
use linux_lx_dos::command;
use linux_lx_dos::utils::args::{Args, Commands};
fn main() -> Result<(), linux_lx_dos::LxDosError> {
    let args = Args::parse();

    let log_level = if args.quiet {
        log::LevelFilter::Off
    } else if args.debug {
        log::LevelFilter::Debug
    } else if args.verbose {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Warn
    };

    env_logger::builder().filter_level(log_level).init();

    match args.command {
        Commands::Start => command::start(),
        Commands::Stop => command::stop(),
        Commands::Welcome => command::welcome(),
        Commands::Run => command::run(),
    }
}
