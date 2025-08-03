use clap::Parser;
use linux_lx_dos::command;
use linux_lx_dos::utils::args::{Args, Commands, InnerArgs, InnerSubCommands};

fn main() -> Result<(), linux_lx_dos::LxDosError> {
    if is_frontend() { frontend() } else { backend() }
}

fn is_frontend() -> bool {
    if let Ok(lxdos_backend_var) = std::env::var("LXDOS_BACKEND") {
        lxdos_backend_var.parse::<usize>().is_err()
    } else {
        true
    }
}

fn frontend() -> Result<(), linux_lx_dos::LxDosError> {
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
    }
}
fn backend() -> Result<(), linux_lx_dos::LxDosError> {
    let args = InnerArgs::parse();
    match args.command {
        InnerSubCommands::Window { window_type } => {
            command::backend::window(&args.pipe_name, window_type)
        }
    }
}
