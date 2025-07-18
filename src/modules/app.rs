use super::lx_dos::LxDos;
use crate::command;
use crate::utils::args::Args;
use crate::utils::args::Commands;
use crate::LxDosError;
pub mod gui;
#[derive(Default, Debug)]
pub struct App {
    pub lx_dos: LxDos,
    pub gui: gui::Gui,
}

impl App {
    pub fn exec(&self, args: Args) -> Result<(), LxDosError> {
        println!("{:#?}", self.lx_dos);
        match args.command {
            Commands::Start => command::start(),
            Commands::Stop => command::stop(),
            Commands::Welcome => command::welcome(),
            Commands::Run => command::run(),
        }
    }
}
