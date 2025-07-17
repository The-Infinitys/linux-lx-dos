use clap::Parser;
use linux_lx_dos::modules::app::App;
use linux_lx_dos::utils::args::Args;
fn main() -> Result<(), linux_lx_dos::LxDosError> {
    let args = Args::parse();

    let log_level = if args.quiet {
        log::LevelFilter::Off
    } else if args.debug {
        log::LevelFilter::Debug
    } else {
        match args.verbose {
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Warn,
            _ => log::LevelFilter::Trace,
        }
    };

    env_logger::builder().filter_level(log_level).init();

    let app = App::default();
    app.exec(args)
}
