use super::lx_dos::LxDos;
use crate::LxDosError;
use crate::command;
use crate::utils::args::Args;
use crate::utils::args::Commands;
use gui::builders::ApplicationWindowBuilder;
use gui::gio::prelude::ApplicationExt;
use gui::glib;
use system_tray::SystemTray;

// GUIスレッドに送信するコマンドを定義します。
enum GuiCommand {
    Quit,
}

// 各GUIアプリケーションの管理に必要な要素を保持する構造体です。
struct GuiManager {
    quit_sender: std::sync::mpsc::Sender<GuiCommand>,
    application: gui::Application,
}

pub struct App {
    pub lx_dos: LxDos,
    pub tray: SystemTray,
    // GUIアプリケーションとその終了チャネルを管理します。
    gui_managers: Vec<GuiManager>,
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

    // GUIアプリケーションをビルドします。
    pub fn gui_app() -> gui::Application {
        let flags = gui::gio::ApplicationFlags::HANDLES_OPEN;
        let gui = gui::Application::builder()
            .application_id(&Self::app_id())
            .flags(flags)
            .build();
        gui
    }

    pub fn add_gui(&mut self, gui: gui::Application) {
        let (quit_sender, quit_receiver) = std::sync::mpsc::channel::<GuiCommand>();
        let app_clone = gui.clone();
        glib::MainContext::default().spawn_local(async move {
            while let Ok(cmd) = quit_receiver.recv() {
                match cmd {
                    GuiCommand::Quit => {
                        app_clone.quit();
                        break;
                    }
                }
            }
        });

        // Store the manager for this GUI application
        self.gui_managers.push(GuiManager {
            quit_sender,
            application: gui,
        });
    }

    pub fn window_builder(gui: &gui::Application, title: &str) -> ApplicationWindowBuilder {
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

        ApplicationWindow::builder().application(gui).title(title)
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            lx_dos: LxDos::default(),
            tray: Self::system_tray(),
            gui_managers: Vec::new(),
        }
    }
}

/// `App`構造体がスコープを外れてドロップされる際に、管理しているすべてのGUIアプリケーションを終了します。
impl Drop for App {
    fn drop(&mut self) {
        for manager in self.gui_managers.drain(..) {
            // 各GUIアプリケーションに終了コマンドを送信します。
            if let Err(e) = manager.quit_sender.send(GuiCommand::Quit) {
                eprintln!("GUI終了コマンドの送信に失敗しました: {:?}", e);
            }
            // アプリケーションを明示的に終了します。
            manager.application.quit();
        }
    }
}