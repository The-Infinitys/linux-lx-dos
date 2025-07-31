use crate::LxDosError;
use crate::modules::app::App;
use slint::ComponentHandle;
use system_tray::{Event as SystemTrayEvent, Menu as SystemTrayMenu};
mod ui {
    slint::include_modules!();
}
pub fn run() -> Result<(), LxDosError> {
    // Appインスタンスを可変にする必要があります。なぜなら、add_guiメソッドがAppの状態を変更するためです。
    // let app = App::default();

    // システムトレイのアイコンとメニューを設定します。
    let tray = App::system_tray()
        .icon(include_bytes!("../../public/icon.svg"), "svg")
        .menu(SystemTrayMenu::new("Open".to_string(), "open".to_string()))
        .menu(SystemTrayMenu::new("Quit".to_string(), "quit".to_string()));
    tray.start();
    fn handle_open() -> Result<(), LxDosError> {
        let main_window = ui::MainWindow::new()?;
        main_window.run()?;
        println!("Open");
        Ok(())
    }
    // システムトレイからのイベントを処理するループ。
    loop {
        match tray.poll_event()? {
            SystemTrayEvent::MenuItemClicked(id) => match id.as_str() {
                "open" => handle_open()?,
                "quit" => {
                    // 「Quit」がクリックされたらループを終了します。
                    println!("LxDosアプリケーションを終了します。");
                    break;
                }
                _ => {} // その他のメニュー項目は無視します。
            },
            SystemTrayEvent::TrayClicked => handle_open()?,
            _ => {} // その他のシステムトレイイベントは無視します。
        }
    }

    // ループが終了したらシステムトレイを停止します。
    tray.stop();
    Ok(())
}
