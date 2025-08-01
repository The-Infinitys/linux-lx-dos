use crate::LxDosError;
use crate::command;
pub mod messages;
use crate::utils::args::Args;
use crate::utils::args::Commands;
use system_tray::SystemTray;
pub mod gui;
pub struct App {}

impl App {
    pub fn exec(&self, args: Args) -> Result<(), LxDosError> {
        match args.command {
            Commands::Start => command::start(),
            Commands::Stop => command::stop(),
            Commands::Welcome => command::welcome(),
        }
    }

    pub fn system_tray() -> SystemTray {
        SystemTray::new(&Self::organization(), &Self::app_id())
    }

    pub fn organization() -> String {
        "LxDos".to_string()
    }

    pub fn app_id() -> String {
        "com.the-infinitys.lx-dos".to_string()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {}
    }
}

/// `App`構造体がスコープを外れてドロップされる際に、管理しているすべてのGUIアプリケーションを終了します。
impl Drop for App {
    fn drop(&mut self) {}
}
