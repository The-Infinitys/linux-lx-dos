use crate::LxDosError;
use crate::modules::app::App;
use system_tray::Event as TrayEvent;
use system_tray::Menu as TrayMenu;

pub fn start() -> Result<(), LxDosError> {
    let mut app = App::default();
    app.windows.start_server()?;

    let tray = App::system_tray()
        .menu(TrayMenu::new("Open".to_string(), "open".to_string()))
        .menu(TrayMenu::new("Quit".to_string(), "quit".to_string()));
    tray.start();

    loop {
        match tray.poll_event()? {
            TrayEvent::MenuItemClicked(id) => match id.as_str() {
                "open" => {
                    app.windows.open_window()?;
                }
                "quit" => break,

                _ => {}
            },
            TrayEvent::TrayClicked => {
                app.windows.open_window()?;
            }
            _ => {}
        }
    }
    Ok(())
}
