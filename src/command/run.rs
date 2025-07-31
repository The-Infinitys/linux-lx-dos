use crate::LxDosError;
use crate::modules::app::App; // App structure is assumed to be defined at this path
use gui::gio::prelude::ApplicationExtManual;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() -> Result<(), LxDosError> {
    // Rc<RefCell<App>> で app をラップし、共有所有権を可能にする
    let app_rc = Rc::new(RefCell::new(App::default()));

    // メインアプリケーションのセットアップ用に gui の参照をクローンする
    let gui_main = app_rc.borrow().gui.clone();

    // 'open' シグナルをウィンドウの作成と表示に接続する
    // このクロージャはメインスレッドで実行されるため、app_rc を直接借用できる
    let app_rc_for_connect_open = app_rc.clone(); // connect_open クロージャ用に app_rc をクローン
    gui_main.connect_open(move |_gui_app, _f, _hint| {
        println!("GTK Application 'open' signal received. Creating window..."); // デバッグ出力
        use gui::prelude::*;
        // RefCell から app を借用して使用する
        let app_ref = app_rc_for_connect_open.borrow();
        let window = app_ref
            .window_builder("hello")
            .width_request(800)
            .height_request(600)
            .build();
        let button = gui::Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        window.set_child(Some(&button));
        window.present();
        println!("Window created and presented."); // デバッグ出力
    });

    gui_main.run();

    Ok(())
}
