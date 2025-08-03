use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use super::App;
use gui::{
    builders::ApplicationWindowBuilder,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
};
use tempfile::TempDir;

// Gui構造体を修正し、TempDirを保持して一時ファイルが削除されないようにします。
pub struct Gui {
    gui: gui::Application,
    #[allow(unused)]
    resource_path: PathBuf,
    _temp_dir: TempDir, // 一時ディレクトリのハンドルを保持
}

impl Default for Gui {
    fn default() -> Self {
        Self::new()
    }
}

impl Gui {
    // GUIアプリケーションをビルドします。
    pub fn new() -> Self {
        let flags = gui::gio::ApplicationFlags::HANDLES_OPEN;

        // tmpfileクレートを利用して一時ディレクトリを作成
        let temp_dir = TempDir::new().expect("Failed to create temporary directory.");
        let resource_path = temp_dir.path().to_path_buf();

        // アイコンファイルをバイナリとして直接組み込む
        let icon_bytes = include_bytes!("../../../public/icon.svg");

        // 一時ディレクトリ内にアイコンファイルを作成
        let icon_path = resource_path.join(Gui::icon_name());
        {
            // スコープを限定し、ファイルハンドルをすぐにドロップしてファイルをロックしないようにします
            let mut file = File::create(&icon_path).expect("Failed to create temporary icon file.");
            file.write_all(icon_bytes)
                .expect("Failed to write to temporary icon file.");
        }
        println!("{}", icon_path.display());
        let gui = gui::Application::builder()
            .application_id(App::app_id())
            .flags(flags)
            .build();

        // アプリケーションのリソースベースパスとして一時ディレクトリを設定
        // これにより、GTKはicon_name()で指定された名前のファイルをこのパスから探します。
        gui.set_resource_base_path(Some(resource_path.to_str().unwrap()));

        Self {
            gui,
            resource_path,
            _temp_dir: temp_dir,
        }
    }

    fn icon_name() -> String {
        format!("{}-app-icon.svg", App::app_id().to_ascii_lowercase())
    }

    pub fn window_builder(gui: &gui::Application, title: &str) -> ApplicationWindowBuilder {
        use gui::ApplicationWindow;
        use gui::CssProvider;
        use gui::Settings;
        use gui::prelude::*;

        let mut theme_name = "default".to_string();
        if let Some(settings) = Settings::default() {
            theme_name = settings.property::<String>("gtk-theme-name");
        }
        let provider = CssProvider::new();
        provider.load_named(&theme_name, None);
        gui::style_context_add_provider_for_display(
            &gui::gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gui::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        ApplicationWindow::builder()
            .application(gui)
            .title(title)
            .icon_name(Gui::icon_name())
    }

    pub fn handler<F: Fn(&gui::Application) + 'static>(&self, f: F) {
        self.gui.connect_open(move |app, _files, _hint| f(app));
    }

    pub fn run(&self) {
        self.gui.run();
    }
}
