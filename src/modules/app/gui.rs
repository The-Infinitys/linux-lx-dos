use super::App;
use gui::{builders::ApplicationWindowBuilder, gio::prelude::ApplicationExtManual};
pub struct Gui {
    gui: gui::Application,
}
impl Default for Gui {
    fn default() -> Self {
        Self::new()
    }
}

impl Gui {
    // GUIアプリケーションをビルドします。
    pub fn new() -> Self {
        let flags = gui::gio::ApplicationFlags::HANDLES_OPEN;
        let gui = gui::Application::builder()
            .application_id(App::app_id())
            .flags(flags)
            .build();
        Self { gui }
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
    pub fn handler<F: Fn(&gui::Application) + 'static>(&self, f: F) {
        self.gui.connect_open(move |app, _files, _hint| f(app));
    }
    pub fn run(&self) {
        self.gui.run();
    }
}
