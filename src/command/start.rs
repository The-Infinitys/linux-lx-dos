use crate::LxDosError;
use crate::modules::app::App;
use crate::modules::app::instance::InstanceMessage;
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

        // サーバーからのメッセージをポーリング
        match app.windows.poll_event() {
            Ok(messages) => {
                for message in messages {
                    match message {
                        InstanceMessage::OpenWindow { pipe_name } => {
                            println!("Received OpenWindow for pipe: {}", pipe_name);
                        }
                        InstanceMessage::CloseWindow { pipe_name } => {
                            println!("Received CloseWindow for pipe: {}", pipe_name);
                            // ここでウィンドウを閉じる処理を実装可能
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Poll event error: {}", e);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    Ok(())
}