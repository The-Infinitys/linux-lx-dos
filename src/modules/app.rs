use super::lx_dos::LxDos;
use crate::command;
use crate::utils::args::Args;
use crate::utils::args::Commands;
use crate::LxDosError;
pub struct App {
    lx_dos: LxDos,
}

impl App {
    pub fn exec(&self, args: Args) -> Result<(), LxDosError> {
        println!("{:#?}", self.lx_dos);
        match args.command {
            Commands::Start => command::start(),
            Commands::Stop => command::stop(),
        }
    }
}
impl Default for App {
    fn default() -> Self {
        Self {
            lx_dos: LxDos::default(),
        }
    }
}
