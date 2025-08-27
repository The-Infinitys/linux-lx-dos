use crate::modules::app::instance::InstanceMessage;
use crate::modules::app::settings::{Settings, SettingsData};
use async_channel::Sender;
use gui::prelude::*;

pub fn handle_gui(app: &gui::Application, tx_message: &Sender<InstanceMessage>) {
    let window_title = "Lx DOS -Settings";

    // 設定ファイルから読み込む
    let settings = Settings::open();
    let settings_list: Vec<SettingsData> = settings.keys();

    let vbox = gui::Box::new(gui::Orientation::Vertical, 8);
    for data in settings_list {
        let hbox = gui::Box::new(gui::Orientation::Horizontal, 8);
        let desc_label = gui::Label::builder().label(&data.description).build();
        let value_label = gui::Label::builder().label(&data.value).build();
        hbox.append(&desc_label);
        hbox.append(&value_label);
        vbox.append(&hbox);
    }

    let window = crate::modules::app::gui::Gui::window_builder(app, window_title)
        .child(&vbox)
        .width_request(400)
        .height_request(200)
        .build();

    let tx_message_clone = tx_message.clone();
    let pipe_name_for_close = "settings_window".to_string();
    window.connect_close_request(move |window| {
        println!("Window close requested, sending CloseWindow: {}", pipe_name_for_close);
        let _ = tx_message_clone.send_blocking(InstanceMessage::CloseWindow {
            pipe_name: pipe_name_for_close.clone(),
        });
        window.close();
        gui::glib::Propagation::Proceed
    });

    window.present();
}
