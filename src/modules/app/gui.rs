/// src/modules/app/gui.rs
use crate::qt6::{self, QtAppEvent};

const APP_ID: &str = "org.lx-dos.Main";

#[derive(Debug)]
pub struct Gui<'a> {
    qt: qt6::QtApp<'a>,
}

impl Default for Gui<'_> {
    fn default() -> Self {
        Gui {
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
    pub fn run(&mut self) -> Result<(), crate::LxDosError> {
        self.qt.add_tray_menu_item("Open Window", "open_window")?;
        self.qt.add_tray_menu_item("Exit", "exit")?;
        let qt_app_instance = self.qt.start()?;

        println!("Starting Qt event loop...");
        loop {
            match qt_app_instance.poll_event() {
                Ok(event) => match event {
                    QtAppEvent::None => {}
                    QtAppEvent::TrayClicked => {
                        println!("Tray icon clicked!");
                    }
                    QtAppEvent::TrayDoubleClicked => {
                        println!("Tray icon double-clicked!");
                    }
                    QtAppEvent::MenuItemClicked(id) => {
                        println!("Menu item clicked with ID: {}", id);
                        match id.as_str() {
                            "open_window" => {}
                            "exit" => {
                                println!("Sending quit signal to Qt app...");
                                qt_app_instance.quit();
                                println!("Breaking Rust event loop...");
                                break;
                            }
                            _ => {}
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error polling Qt event: {}", e);
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100)); // Prevent busy-waiting
        }
        println!("Rust event loop broken. Gui::run() returning.");
        // Wait for the Qt application thread to finish
        qt_app_instance.join().expect("Failed to join Qt thread");
        Ok(())
    }
}
