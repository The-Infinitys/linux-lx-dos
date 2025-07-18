use crate::LxDosError;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

pub fn run() -> Result<(), LxDosError> {
    let application = Application::builder()
        .application_id("com.example.lx-dos-background")
        .build();

    application.connect_activate(|app| {
        // GTKテーマの自動適用
        if let Some(settings) = gtk::Settings::default() {
            let theme_name = settings.property::<String>("gtk-theme-name");
            println!("Applying GTK theme: {}", theme_name);
            // GTK4では、gtk-theme-nameプロパティを設定するだけで自動的にテーマが適用されます。
            // 明示的に何かをする必要は通常ありません。
        }

        // バックグラウンドプロセスとして動作するため、ウィンドウは非表示
        let window = ApplicationWindow::builder()
            .application(app)
            .title("LX-DOS Background")
            .default_width(1)
            .default_height(1)
            .visible(false) // ウィンドウを非表示にする
            .build();
        window.show();
    });

    application.run();

    Ok(())
}