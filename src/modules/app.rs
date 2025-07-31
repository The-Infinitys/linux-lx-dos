use gui::builders::ApplicationWindowBuilder;
use system_tray::SystemTray;

use super::lx_dos::LxDos;
use crate::LxDosError;
use crate::command;
use crate::utils::args::Args;
use crate::utils::args::Commands;

pub struct App {
    pub lx_dos: LxDos,
    pub system_tray: SystemTray,
    pub gui: gui::Application,
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
    pub fn window_builder(&self, title: &str) -> ApplicationWindowBuilder {
        use gui::ApplicationWindow;
        use gui::CssProvider;
        use gui::Settings;
        use gui::prelude::*;

        let mut theme_name = "default".to_string();
        if let Some(settings) = Settings::default() {
            theme_name = settings.property::<String>("gtk-theme-name");
        }
        let provider = CssProvider::new();
        provider.load_named(&theme_name, None);
        gui::style_context_add_provider_for_display(
            &gui::gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gui::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        ApplicationWindow::builder()
            .application(&self.gui)
            .title(title)
    }
    // pub fn run_with_args(&self, args: Vec<String>) {
    //     self.gtk.run_with_args(&args);
    // }
}
impl Default for App {
    fn default() -> Self {
        let app_id = "com.the-infinitys.lx-dos";
        let flags = gui::gio::ApplicationFlags::HANDLES_OPEN;
        Self {
            lx_dos: LxDos::default(),
            system_tray: SystemTray::new("LxDos", app_id),
            gui: gui::Application::builder()
                .application_id(app_id)
                .flags(flags)
                .build(),
        }
    }
}
