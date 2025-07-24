// use gtk::prelude::*;
use gtk::prelude::*;
use gtk::{
    builders::ApplicationWindowBuilder,
    gio::{
        prelude::{ApplicationExt, ApplicationExtManual},
        ApplicationFlags, File,
    },
    glib::ExitCode,
    Application, ApplicationWindow, CssProvider, Settings,
};

use crate::qt6::{self, QtAppEvent};

const APP_ID: &str = "org.lx-dos.Main";

#[derive(Debug)]
pub struct Gui<'a> {
    gtk: Application,
    qt: qt6::QtApp<'a>,
}
impl Default for Gui<'_> {
    fn default() -> Self {
        Gui {
            gtk: Application::builder()
                .application_id(APP_ID)
                .flags(ApplicationFlags::HANDLES_OPEN)
                .build(),
            qt: qt6::QtApp::new()
                .with_id(APP_ID)
                .expect("Failed to build QtApp")
                .with_tray()
                .with_icon(include_bytes!("../../../public/icon.svg"), "SVG")
                .expect("Failed to insert icon"),
        }
    }
}
impl Gui<'_> {
    pub fn connect_activate<F: Fn(&Application) + 'static>(&self, f: F) {
        self.gtk.connect_activate(f);
    }
    pub fn connect_open<F: Fn(&Application, &[File], &str) + 'static>(&self, f: F) {
        self.gtk.connect_open(f);
    }
    pub fn window_builder(&self, title: &str) -> ApplicationWindowBuilder {
        let mut theme_name = "default".to_string();
        if let Some(settings) = Settings::default() {
            theme_name = settings.property::<String>("gtk-theme-name");
        }
        let provider = CssProvider::new();
        provider.load_named(&theme_name, None);
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

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

    pub fn run_qt_app(&self) -> Result<(), crate::LxDosError> {
        // Initialize tray and add menu items
        self.qt.add_tray_menu_item("Open", 1001)?;
        self.qt.add_tray_menu_item("Exit", 1002)?;
        let qt_app_instance = self.qt.start()?;

        // Poll for events in the main thread (or another dedicated thread)
        // This is a simplified example; in a real application, you'd want a more robust event loop.
        loop {
            match qt_app_instance.poll_event() {
                Ok(event) => {
                    match event {
                        QtAppEvent::None => {}
                        QtAppEvent::TrayClicked => {
                            println!("Tray icon clicked!");
                        }
                        QtAppEvent::TrayDoubleClicked => {
                            println!("Tray icon double-clicked!");
                        }
                        QtAppEvent::MenuItemClicked(id) => {
                            println!("Menu item clicked with ID: {}", id);
                            if id == 1002 {
                                // Exit menu item
                                qt_app_instance.quit();
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error polling Qt event: {}", e);
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100)); // Prevent busy-waiting
        }

        Ok(())
    }
}
