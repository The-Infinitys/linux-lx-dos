use crate::LxDosError;
use crate::modules::app::gui::Gui;
use crate::modules::app::instance::{InstanceMessage, WindowType};
use async_channel::Sender;
use gui::prelude::*;

pub fn window(pipe_name: &str, _window_type: WindowType) -> Result<(), LxDosError> {
    let mut gui = Gui::new();

    let pipe_name_str = pipe_name.to_string();
    gui.handler(
        move |app: &gui::Application, tx_message: &Sender<InstanceMessage>| {
            let window_title = "Lx DOS";
            let button = gui::Button::builder()
                .label("Press me!")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();

            let window = Gui::window_builder(app, window_title)
                .child(&button)
                .width_request(480)
                .height_request(360)
                .build();

            let window_weak = window.downgrade();
            button.connect_clicked(move |_| {
                if let Some(window) = window_weak.upgrade() {
                    println!("Button clicked, closing window");
                    window.close();
                }
            });

            let tx_message_clone = tx_message.clone();
            let pipe_name_for_close = pipe_name_str.clone();
            window.connect_close_request(move |window| {
                println!(
                    "Window close requested, sending CloseWindow: {}",
                    pipe_name_for_close
                );
                let _ = tx_message_clone.send_blocking(InstanceMessage::CloseWindow {
                    pipe_name: pipe_name_for_close.clone(),
                });
                window.close();
                gui::glib::Propagation::Proceed
            });

            app.connect_window_added(|_, window| {
                println!("Window added to application");
                window.present();
            });

            app.connect_window_removed(|app, _| {
                println!("Window removed from application");
                if app.windows().is_empty() {
                    app.quit();
                }
            });

            // Activate時にOpenWindowメッセージを送信
            let tx_for_activate = tx_message.clone();
            let pipe_name_clone_for_activate = pipe_name_str.clone();
            println!("Application activated, sending OpenWindow message.");
            let _ = tx_for_activate.send_blocking(InstanceMessage::OpenWindow {
                pipe_name: pipe_name_clone_for_activate,
                window_type: WindowType::Main,
            });

            window.present();
        },
    );

    gui.on_message(|app, message| {
        match message {
            InstanceMessage::OpenWindow {
                pipe_name,
                window_type,
            } => {
                println!(
                    "Received OpenWindow for pipe: {}, type: {:?}",
                    pipe_name, window_type
                );
                // 必要ならここで新しいウィンドウを開く
            }
            InstanceMessage::CloseWindow { pipe_name } => {
                println!("Received CloseWindow for pipe: {}", pipe_name);
                app.quit();
            }
            InstanceMessage::MaximizeWindow { pipe_name } => {
                println!("Received MaximizeWindow for pipe: {}", pipe_name);
                if let Some(window) = app.active_window() {
                    window.maximize();
                }
            }
            InstanceMessage::MinimizeWindow { pipe_name } => {
                println!("Received MinimizeWindow for pipe: {}", pipe_name);
                if let Some(window) = app.active_window() {
                    window.minimize();
                }
            }
            InstanceMessage::RestoreWindow { pipe_name } => {
                println!("Received RestoreWindow for pipe: {}", pipe_name);
                if let Some(window) = app.active_window() {
                    window.unmaximize();
                    window.present();
                }
            }
        }
    });

    gui.run(pipe_name);

    println!("Application closed");
    Ok(())
}
