use crate::LxDosError;
use crate::modules::app::gui::Gui;
use crate::modules::app::instance::{InstanceMessage, WindowType};
use gui::prelude::*;

pub fn window(pipe_name: &str, _window_type: WindowType) -> Result<(), LxDosError> {
    let mut gui = Gui::new();

    // GUIスレッドでバックエンドからのメッセージを処理
    gui.on_message(|message| {
        match message {
            InstanceMessage::OpenWindow {
                pipe_name,
                window_type,
            } => {
                println!(
                    "Received OpenWindow for pipe: {}, type: {:?}",
                    pipe_name, window_type
                );
                // ここでウィンドウを開くなどのGUI操作を実行
            }
            InstanceMessage::CloseWindow { pipe_name } => {
                println!("Received CloseWindow for pipe: {}", pipe_name);
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

    gui.handler(
        move |app: &gui::Application, _tx_message: &async_channel::Sender<InstanceMessage>| {
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

            window.connect_close_request(move |window| {
                println!("Window close requested.");
                window.close();
                gui::glib::Propagation::Proceed
            });

            app.connect_window_added(|_, window| {
                println!("Window added to application");
                window.present();
            });

            app.connect_window_removed(|app, _| {
                println!("Window removed from application");
                app.quit();
            });
        },
    );

    gui.run(pipe_name);

    Ok(())
}
