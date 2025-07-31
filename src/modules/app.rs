use super::lx_dos::LxDos;
use crate::LxDosError;
use crate::command;
use crate::utils::args::Args;
use crate::utils::args::Commands;
use system_tray::SystemTray;

// 各GUIアプリケーションの管理に必要な要素を保持する構造体です。

pub struct App {
    pub lx_dos: LxDos,
    pub tray: SystemTray,
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
        Self {
            lx_dos: LxDos::default(),
            tray: Self::system_tray(),
        }
    }
}

/// `App`構造体がスコープを外れてドロップされる際に、管理しているすべてのGUIアプリケーションを終了します。
impl Drop for App {
    fn drop(&mut self) {}
}
