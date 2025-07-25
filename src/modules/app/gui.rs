/// src/modules/app/gui.rs
use crate::qt6::{self, QtAppEvent};
use gtk::prelude::*;
use gtk::{
    gio::{
        prelude::{ApplicationExt, ApplicationExtManual},
        ApplicationFlags, File,
    },
    Application, ApplicationWindow, CssProvider, Settings,
};
use gtk::glib::{self, ControlFlow}; // glib for ControlFlow and idle_add_local
use std::sync::{mpsc, Arc, Mutex}; // For multi-producer, single-consumer channel, Arc, and Mutex

const APP_ID: &str = "org.lx-dos.Main";

#[derive(Debug)]
// Gui構造体からライフタイムパラメータを削除
pub struct Gui {
    gtk: Application, // GTKアプリケーションインスタンスを保持
    // QtAppのライフタイムパラメータを'staticと明示的に指定
    // これは、QtAppが内部で'staticなデータのみを借用していることを前提とします。
    qt: Arc<Mutex<qt6::QtApp<'static>>>,
}

impl Default for Gui {
    fn default() -> Self {
        let gtk_app = Application::builder()
            .application_id(APP_ID)
            .flags(ApplicationFlags::HANDLES_OPEN)
            .build();

        Gui {
            gtk: gtk_app,
            // QtApp::new()を呼び出す際に、ライフタイムパラメータを推論させるか、
            // 必要であれば明示的に<'static>を指定する。
            // ここでは推論に任せますが、もしエラーが出る場合は qt6::QtApp::<'static>::new() を試してください。
            qt: Arc::new(Mutex::new(qt6::QtApp::new()
                .with_id(APP_ID)
                .expect("Failed to build QtApp")
                .with_tray()
                .with_icon(include_bytes!("../../../public/icon.svg"), "SVG")
                .expect("Failed to insert icon"))),
        }
    }
}

impl Gui { // implブロックからもライフタイムパラメータを削除
    /// GTKアプリケーションのactivateシグナルに接続します。
    pub fn connect_activate<F: Fn(&Application) + 'static>(&self, f: F) {
        self.gtk.connect_activate(f);
    }

    /// GTKアプリケーションのopenシグナルに接続します。
    pub fn connect_open<F: Fn(&Application, &[File], &str) + 'static>(&self, f: F) {
        self.gtk.connect_open(f);
    }

    /// アプリケーションのメインイベントループを実行します。
    /// GTKのactivateシグナルハンドラ内でQtのイベントループをセットアップします。
    pub fn run(&mut self) -> Result<(), crate::LxDosError> {
        // QtAppのArc<Mutex>をクローンし、activateハンドラに移動できるようにします。
        // これにより、複数のクロージャが同じQtAppインスタンスを共有できます。
        let qt_app_arc_clone = Arc::clone(&self.qt);

        // GTKアプリケーションのactivateシグナルにハンドラを接続します。
        // `move` キーワードにより、qt_app_arc_clone の所有権がクロージャに移ります。
        self.gtk.connect_activate(move |app| {
            println!("GTKアプリケーションがアクティブになりました。");

            // GTKアプリケーションのCSSプロバイダーをグローバルに設定します
            let mut theme_name = "default".to_string();
            if let Some(settings) = Settings::default() {
                theme_name = settings.property::<String>("gtk-theme-name");
            }
            let provider = CssProvider::new();
            provider.load_named(&theme_name, None);
            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().expect("Could not connect to a display."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            // ここからQt関連のセットアップを開始します。
            // Arc<Mutex>をロックしてQtAppインスタンスにアクセスします。
            let mut qt_app_guard = qt_app_arc_clone.lock().unwrap();

            if let Err(e) = qt_app_guard.add_tray_menu_item("Open", "open_menu_item") {
                eprintln!("Failed to add Qt tray menu item 'Open': {}", e);
                return; // エラーが発生したら処理を中断
            }
            if let Err(e) = qt_app_guard.add_tray_menu_item("Exit", "exit_menu_item") {
                eprintln!("Failed to add Qt tray menu item 'Exit': {}", e);
                return; // エラーが発生したら処理を中断
            }

            // QtAppをスタートし、そのハンドルを取得します。
            // ここでqt_app_guardはスコープを抜けてロックが解放されます。
            let qt_app_started_instance = match qt_app_guard.start() {
                Ok(instance) => instance,
                Err(e) => {
                    eprintln!("Failed to start Qt application: {}", e);
                    return; // エラーが発生したら処理を中断
                }
            };
            let qt_handle_for_quit_signal = qt_app_started_instance.get_handle(); 

            // std::sync::mpsc::channel を使用してスレッド間でイベントを送信
            let (tx, rx) = mpsc::channel::<QtAppEvent>();

            // 別スレッドでQtイベントをポーリングします
            std::thread::spawn(move || {
                let qt_app_instance_in_thread = qt_app_started_instance; // 所有権をスレッドにムーブ
                loop {
                    match qt_app_instance_in_thread.poll_event() {
                        Ok(event) => {
                            // イベントをmpscチャネル経由で送信します
                            if let Err(_) = tx.send(event.clone()) {
                                eprintln!("Failed to send Qt event to GTK main thread (receiver dropped)");
                                break; // レシーバーがドロップされた場合、スレッドを終了
                            }
                            // 終了イベントの場合、Qtアプリに終了をシグナルし、ポーリングスレッドを終了
                            if let QtAppEvent::MenuItemClicked(id) = event {
                                if id == "exit_menu_item" {
                                    qt_app_instance_in_thread.quit();
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("バックグラウンドスレッドでQtイベントのポーリング中にエラーが発生しました: {}", e);
                            break;
                        }
                    }
                    // CPU使用率を抑えるために短い時間スリープします
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            });

            let gtk_app_clone = app.clone(); // connect_activateの引数`app`をクローン
            let qt_handle_for_gtk_quit = qt_handle_for_quit_signal;

            // glib::idle_add_local を使用して、GTKメインスレッドでmpscチャネルを定期的にチェック
            glib::idle_add_local(move || {
                // try_recv() で非ブロッキングにイベントを取得し、キューにあるすべてのイベントを処理します
                while let Ok(event) = rx.try_recv() { 
                    match event {
                        QtAppEvent::None => {}
                        QtAppEvent::TrayClicked => {
                            println!("トレイアイコンがクリックされました！ (GTKスレッド)");
                        }
                        QtAppEvent::TrayDoubleClicked => {
                            println!("トレイアイコンがダブルクリックされました！ (GTKスレッド)");
                        }
                        QtAppEvent::MenuItemClicked(id_str) => {
                            println!("メニュー項目がクリックされました (ID: {}) (GTKスレッド)", id_str);
                            match id_str.as_str() {
                                "exit_menu_item" => {
                                    gtk_app_clone.quit();
                                    unsafe {
                                        qt6::bind::quit_qt_app(qt_handle_for_gtk_quit.as_ptr());
                                    }
                                    return ControlFlow::Break; // GTKアイドルコールバックを停止
                                }
                                "open_menu_item" => {
                                    println!("Openメニュー項目がクリックされました！");
                                    // GTKウィンドウの作成と表示
                                    let window = ApplicationWindow::builder()
                                        .application(&gtk_app_clone)
                                        .title("新しいGTKウィンドウ")
                                        .default_width(800)
                                        .default_height(600)
                                        .build();
                                    window.set_visible(true); // GTKウィンドウを表示
                                }
                                _ => {
                                    println!("不明なメニュー項目がクリックされました: {}", id_str);
                                }
                            }
                        }
                    }
                }
                ControlFlow::Continue // 引き続きアイドルコールバックを呼び出す
            });
        });

        // GTKアプリケーションを実行します。これはメインスレッドをブロックし、イベントを処理します。
        // activateシグナルハンドラが呼び出され、その中でQtのセットアップとイベント処理が開始されます。
        self.gtk.run();

        Ok(())
    }
}
