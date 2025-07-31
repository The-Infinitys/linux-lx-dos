use crate::LxDosError;
use crate::modules::app::App;
use gui::{gio::prelude::ApplicationExt, prelude::GtkWindowExt};
use system_tray::{Event as SystemTrayEvent, Menu as SystemTrayMenu};

pub fn run() -> Result<(), LxDosError> {
    // Appインスタンスを可変にする必要があります。なぜなら、add_guiメソッドがAppの状態を変更するためです。
    let mut app = App::default();

    // システムトレイのアイコンとメニューを設定します。
    let tray = App::system_tray()
        .icon(include_bytes!("../../public/icon.svg"), "svg")
        .menu(SystemTrayMenu::new("Open".to_string(), "open".to_string()))
        .menu(SystemTrayMenu::new("Quit".to_string(), "quit".to_string()));
    tray.start();

    // システムトレイからのイベントを処理するループ。
    loop {
        match tray.poll_event()? {
            SystemTrayEvent::MenuItemClicked(id) => match id.as_str() {
                "open" => {
                    // 新しいGUIアプリケーションインスタンスを作成します。
                    let gui = App::gui_app();
                    gui.connect_activate(|gui| {
                        // ウィンドウビルダーを使用してウィンドウを作成し、設定します。
                        let window = App::window_builder(&gui, "LxDos GUI") // ウィンドウタイトルをより具体的に
                            .width_request(800)
                            .height_request(600)
                            .build();

                        // ウィンドウを表示します。
                        window.present();
                    });
                    app.add_gui(gui)?;
                }
                "quit" => {
                    // 「Quit」がクリックされたらループを終了します。
                    println!("LxDosアプリケーションを終了します。");
                    break;
                }
                _ => {} // その他のメニュー項目は無視します。
            },
            SystemTrayEvent::TrayClicked => {
                println!("トレイアイコンがクリックされました！");
            }
            _ => {} // その他のシステムトレイイベントは無視します。
        }
    }

    // ループが終了したらシステムトレイを停止します。
    tray.stop();
    Ok(())
}
