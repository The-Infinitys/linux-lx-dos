use gtk::prelude::*;
use gtk::{
    builders::ApplicationWindowBuilder,
    gio::{
        prelude::{ApplicationExt, ApplicationExtManual},
        ApplicationFlags, File,
    },
    glib::ExitCode,
    Application, ApplicationWindow, Settings,
};

const APP_ID: &str = "org.lx-dos.Main";

#[derive(Debug, Clone)]
pub struct Gui {
    gtk: Application,
}
impl Default for Gui {
    fn default() -> Self {
        Gui {
            gtk: Application::builder()
                .application_id(APP_ID)
                .flags(ApplicationFlags::HANDLES_OPEN)
                .build(),
        }
    }
}
impl Gui {
    pub fn connect_activate<F: Fn(&Application) + 'static>(&self, f: F) {
        self.gtk.connect_activate(move |app| {
            if let Some(settings) = Settings::default() {
                let theme_name = settings.property::<String>("gtk-theme-name");
                log::error!("Current GTK theme: {}", theme_name);
            }
            f(app);
        });
    }
    pub fn connect_open<F: Fn(&Application, &[File], &str) + 'static>(&self, f: F) {
        self.gtk.connect_open(f);
    }
    pub fn window_builder(&self, title: &str) -> ApplicationWindowBuilder {
        ApplicationWindow::builder()
            .application(&self.gtk)
            .title(title)
    }
    pub fn run_with_args(&self, args: Vec<String>) {
        self.gtk.run_with_args(&args);
    }
    pub fn run(&self) -> ExitCode {
        self.gtk.run()
    }
}
