use crate::LxDosError;
use crate::modules::app::gui::Gui;
use crate::modules::app::instance::{InstanceMessage, WindowClient};
use instance_pipe::Client;
use std::thread;
use std::time::Duration;

pub fn run_backend(pipe_name: &str) -> Result<(), LxDosError> {
    let client = Client::start(pipe_name)?;
    let window_client = WindowClient::new(client);
    let window_client_clone = window_client.clone();
    let gui = Gui::new();
    let pipe_name = pipe_name.to_string();

    // スレッドでクライアントのイベントポーリングを実行
    let client_handle = thread::spawn(move || {
        loop {
            match window_client_clone.poll_event() {
                Ok(messages) => {
                    for message in messages {
                        match message {
                            InstanceMessage::OpenWindow { pipe_name } => {
                                println!("Client received OpenWindow for pipe: {}", pipe_name);
                            }
                            InstanceMessage::CloseWindow { pipe_name } => {
                                println!("Client received CloseWindow for pipe: {}", pipe_name);
                                // ここでウィンドウを閉じる処理を実装可能
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Client poll error: {}", e);
                    break;
                }
            }
            thread::sleep(Duration::from_millis(50));
        }
        Ok::<(), LxDosError>(())
    });

    let pipe_name_clone = pipe_name.clone();
    gui.handler(move |app| {
        use gui::Button;
        use gui::prelude::*;
        let button = Button::builder()
            .label("Press me!")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        let pipe_name_clone = pipe_name_clone.clone();
        let window_client_clone = window_client.clone();
        button.connect_clicked(move |_| {
            println!("Button clicked, sending CloseWindow: {}", pipe_name_clone);
            if let Err(e) = window_client_clone.send(&InstanceMessage::CloseWindow {
                pipe_name: pipe_name_clone.clone(),
            }) {
                eprintln!("Failed to send CloseWindow: {}", e);
            }
        });

        let window = Gui::window_builder(app, "Lx-DOS Window")
            .child(&button)
            .width_request(480)
            .height_request(360)
            .build();
        window.present();
    });

    gui.run();
    client_handle.join().map_err(|e| LxDosError::Message(format!("Client thread panicked: {:?}", e)))??;
    Ok(())
}