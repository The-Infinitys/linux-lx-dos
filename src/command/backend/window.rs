use crate::LxDosError;
use crate::modules::app::gui::Gui;
use crate::modules::app::instance::{InstanceMessage, WindowType};
use async_channel::Sender;
use gui::prelude::*;
mod main_window;
mod settings_window;
mod welcome_window;
pub fn window(pipe_name: &str, window_type: WindowType) -> Result<(), LxDosError> {
    let mut gui = Gui::new();
    let pipe_name_str = pipe_name.to_string();
    gui.handler(
        move |app: &gui::Application, tx_message: &Sender<InstanceMessage>| {
            // 共通: ウィンドウ追加・削除時の処理
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

            // Activate時にOpenWindowメッセージを送信（Mainのみ）
            if window_type == WindowType::Main {
                let tx_for_activate = tx_message.clone();
                let pipe_name_clone_for_activate = pipe_name_str.clone();
                println!("Application activated, sending OpenWindow message.");
                let _ = tx_for_activate.send_blocking(InstanceMessage::OpenWindow {
                    pipe_name: pipe_name_clone_for_activate,
                    window_type: WindowType::Main,
                });
            }

            // WindowTypeごとに処理を分岐
            match window_type {
                WindowType::Main => main_window::handle_gui(app, tx_message),
                WindowType::Welcome => welcome_window::handle_gui(app, tx_message),
                WindowType::Settings => settings_window::handle_gui(app, tx_message),
                //_ => {}, // 他のタイプがあればここで追加
            }
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
