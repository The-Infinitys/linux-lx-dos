use crate::LxDosError;
use crate::modules::app::gui::Gui;
use crate::modules::app::instance::{InstanceMessage, WindowType};
use async_channel::Sender;
use gui::prelude::*;

pub fn window(pipe_name: &str, _window_type: WindowType) -> Result<(), LxDosError> {
    let mut gui = Gui::new();

    // 1. `gui.handler` を呼び出して、GUIイベント（ウィンドウの表示など）と
    //    メッセージチャンネルの初期化を行います。
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

            window.present();

            let window_weak = window.downgrade();
            button.connect_clicked(move |_| {
                if let Some(window) = window_weak.upgrade() {
                    println!("Button clicked, closing window");
                    window.close();
                }
            });

            // 閉じる要求時にバックエンドにメッセージを送信する
            let tx_message_clone = tx_message.clone();
            window.connect_close_request(move |_| {
                println!("Window close requested, sending CloseWindow message.");
                let _ = tx_message_clone.send_blocking(InstanceMessage::CloseWindow {
                    pipe_name: "main_window".to_string(), // または適切なパイプ名
                });
                gui::glib::Propagation::Proceed
            });

            app.connect_window_added(|_, window| {
                println!("Window added to application");
                window.present();
            });

            app.connect_window_removed(|app, _| {
                println!("Window removed from application");
                // すべてのウィンドウが閉じられたときにアプリケーションを終了させる
                if app.windows().is_empty() {
                    app.quit();
                    println!("app wuit");
                }
            });
        },
    );
            
    // 2. `gui.on_message` を呼び出して、受信したメッセージのハンドリングロジックを設定します。
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
                // ここで新しいウィンドウを開くなどのGUI操作を実行
            }
            InstanceMessage::CloseWindow { pipe_name } => {
                println!("Received CloseWindow for pipe: {}", pipe_name);
                app.quit();
            }
            InstanceMessage::MaximizeWindow { pipe_name } => {
                println!("Received MaximizeWindow for pipe: {}", pipe_name);
            }
            InstanceMessage::MinimizeWindow { pipe_name } => {
                println!("Received MinimizeWindow for pipe: {}", pipe_name);
            }
            InstanceMessage::RestoreWindow { pipe_name } => {
                println!("Received RestoreWindow for pipe: {}", pipe_name);
            }
        }
    });

    // 3. アプリケーションを実行します。
    //    このメソッド内で、バックエンド通信スレッドも適切に管理されます。
    gui.run(pipe_name);

    println!("Application closed");
    Ok(())
}
