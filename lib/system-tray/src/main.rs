use system_tray::{SystemTray, Menu, Event};

fn main() {
    let mut tray = SystemTray::new("system-tray-test", "com.example.system-tray-test");

    let icon_data = include_bytes!("../icon.svg");

    tray.icon(icon_data, "svg")
        .menu(Menu::new("Quit".to_string(), "quit".to_string()));

    tray.handle_event(|event| match event {
        Event::Menu(id) => {
            if id == "quit" {
                std::process::exit(0);
            }
        }
        _ => {},
    });

    tray.run();
}
