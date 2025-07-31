use super::super::modules::app::App;
use crate::LxDosError;
use system_tray::Event as SystemTrayEvent;
use system_tray::Menu as SystemTrayMenu;
pub fn run() -> Result<(), LxDosError> {
    let mut app = App::default();
    app.system_tray = app
        .system_tray
        .icon(include_bytes!("../../public/icon.svg"), "svg")
        .menu(SystemTrayMenu::new("Open".to_string(), "open".to_string()))
        .menu(SystemTrayMenu::new(
            "Quit App".to_string(),
            "quit".to_string(),
        ));
    app.system_tray.start();
    fn open() {
        println!("Open");
    }
    fn sleep(millis: u64) {
        let dur = std::time::Duration::from_millis(millis);
        std::thread::sleep(dur);
    }
    loop {
        match app.system_tray.poll_event()? {
            SystemTrayEvent::MenuItemClicked(id) => match id.as_str() {
                "open" => open(),
                "quit" => break,
                _ => {}
            },
            SystemTrayEvent::TrayClicked => open(),
            _ => {}
        }
        sleep(50);
    }
    app.system_tray.stop();
    Ok(())
}
