use super::lx_dos::LxDos;
use crate::LxDosError;
use crate::command;
use crate::utils::args::Args;
use crate::utils::args::Commands;
mod gui;
mod run;
pub use gui::*;
#[derive(Debug)]
pub struct App {
    pub lx_dos: LxDos,
    value: u32,
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
impl Default for App {
    fn default() -> Self {
        Self {
            lx_dos: LxDos::default(),
            value: 0,
        }
    }
}
