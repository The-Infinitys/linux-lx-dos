use super::lx_dos::LxDos;
use crate::command;
use crate::utils::args::Args;
use crate::utils::args::Commands;
use crate::LxDosError;
pub mod gui;
mod run;
#[derive(Default, Debug)]
pub struct App<'a> {
    pub lx_dos: LxDos,
    pub gui: gui::Gui<'a>,
}

impl App<'_> {
    pub fn exec(&self, args: Args) -> Result<(), LxDosError> {
        println!("{:#?}", self.lx_dos);
        match args.command {
            Commands::Start => command::start(),
            Commands::Stop => command::stop(),
            Commands::Welcome => command::welcome(),
            Commands::Run => command::run(),
        }
    }
    pub fn run(&mut self) -> Result<(), LxDosError> {
        run::run(self)
    }
}
