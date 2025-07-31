use crate::LxDosError;
use crate::modules::app::App;
// use system_tray::{Event as SystemTrayEvent, Menu as SystemTrayMenu};
pub fn run() -> Result<(), LxDosError> {
    use gui::gio::prelude::*;
    let gui = App::gui_app();
    gui.connect_open(|gui, _f, _hint| {
        use gui::prelude::*;
        let window = App::window_builder(gui, "Hello, World").build();
        window.present();
    });
    gui.run();
    // let tray = App::system_tray()
    //     .icon(include_bytes!("../../public/icon.svg"), "svg")
    //     .menu(SystemTrayMenu::new("Open".to_string(), "open".to_string()))
    //     .menu(SystemTrayMenu::new("Quit".to_string(), "quit".to_string()));
    // tray.start();
    // fn handle_open() -> Result<(), LxDosError> {
    //     Ok(())
    // }
    // // システムトレイからのイベントを処理するループ。
    // loop {
    //     match tray.poll_event()? {
    //         SystemTrayEvent::MenuItemClicked(id) => match id.as_str() {
    //             "open" => handle_open()?,
    //             "quit" => {
    //                 // 「Quit」がクリックされたらループを終了します。
    //                 println!("LxDosアプリケーションを終了します。");
    //                 break;
    //             }
    //             _ => {} // その他のメニュー項目は無視します。
    //         },
    //         SystemTrayEvent::TrayClicked => handle_open()?,
    //         _ => {} // その他のシステムトレイイベントは無視します。
    //     }
    // }

    // // ループが終了したらシステムトレイを停止します。
    // tray.stop();
    Ok(())
}
