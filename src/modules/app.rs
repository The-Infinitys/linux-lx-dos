use system_tray::SystemTray;

use super::lx_dos::LxDos;
use crate::LxDosError;
use crate::command;
use crate::utils::args::Args;
use crate::utils::args::Commands;

pub struct App {
    pub lx_dos: LxDos,
    pub system_tray: SystemTray,
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
            system_tray: SystemTray::new("LxDos", "com.the-infinitys.lx-dos"),
        }
    }
}
