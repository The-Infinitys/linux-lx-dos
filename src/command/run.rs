// src/run.rs

use crate::modules::app::messages::TrayMessage; // 共有メッセージ型をインポート
use crate::LxDosError;
use crate::modules::app::App;
use gui::gio::prelude::*; // gioのトレイトをインポート
use gui::glib; // glibをインポート
use std::io::{self, Read, Write}; // TCP通信のためのI/Oトレイトをインポート
use std::net::{TcpListener, TcpStream}; // TCPリスナーとストリームをインポート
use std::sync::mpsc::{self, Sender}; // 内部チャネル（IPCスレッドからGTKメインスレッドへ）
use std::thread; // スレッド操作をインポート
use std::time::Duration; // ボタンの例のためにインポート

/// プロセス間通信 (IPC) に使用するポート番号。
/// このポートは、システムトレイプロセスとGUIアプリケーションプロセス間で共有されます。
pub const IPC_PORT: u128 = 12345; // ポート番号は u16 で十分ですが、ここでは u128 を使用しています。

/// メインのGTKアプリケーションロジックを実行します。
/// この関数は、システムトレイからのメッセージをプロセス間通信 (IPC) 経由で受信します。
///
/// `start.rs`から直接起動されるか、既存のインスタンスにコマンドを送信するために起動されます。
pub fn run() -> Result<(), LxDosError> {
    let gui_app = App::gui_app();

    // --------------------------------------------------------------------
    // 1. IPCサーバーの初期化とコマンドライン引数の処理
    // --------------------------------------------------------------------
    // IPCスレッドからGTKメインスレッドにメッセージを安全に送るための内部チャネルを作成します。
    let (tx_ipc, rx_ipc) = mpsc::channel::<TrayMessage>();

    // コマンドライン引数を解析し、初期コマンドがあるか確認します。
    // 例: `gui-app-run --command open`
    let args: Vec<String> = std::env::args().collect();
    let initial_command: Option<TrayMessage> = if args.len() > 1 && args[1] == "--command" {
        match args.get(2).map(|s| s.as_str()) {
            Some("open") => Some(TrayMessage::OpenWindow),
            Some("quit") => Some(TrayMessage::QuitApp),
            _ => None, // 不明なコマンドは無視
        }
    } else {
        None // `--command` 引数がない場合
    };

    // TCPリスナーをバインドして、IPCサーバーを起動しようと試みます。
    match TcpListener::bind(format!("127.0.0.1:{}", IPC_PORT)) {
        Ok(listener) => {
            // バインドに成功した場合、このインスタンスがプライマリ（サーバー）になります。
            println!("GTK: Primary instance started. Listening on port {}.", IPC_PORT);

            // IPCサーバーをバックグラウンドスレッドで開始します。
            // このスレッドは、IPCクライアントからの接続を待ち受け、受信したメッセージを
            // `tx_ipc` を介してGTKメインスレッドに送信します。
            // ここでは `gui_app_clone_for_ipc` は不要になりました。
            let tx_ipc_clone = tx_ipc.clone();
            thread::spawn(move || {
                ipc_server_thread(listener, tx_ipc_clone); // gui_app_clone_for_ipc を削除
            });

            // もし起動時にコマンドライン引数で初期コマンドが渡されていたら、それを処理します。
            if let Some(cmd) = initial_command {
                println!("GTK: Processing initial command from args: {:?}", cmd);
                // `glib::idle_add_local` を使ってGTKメインスレッドで処理をスケジュールします。
                // これにより、UIの更新が安全に行われます。
                let gui_app_clone = gui_app.clone(); // クロージャ内で使用するためにクローン
                glib::idle_add_local(move || {
                    match cmd {
                        TrayMessage::OpenWindow => {
                            gui_app_clone.activate(); // アプリケーションをアクティブ化し、ウィンドウを表示
                        }
                        TrayMessage::QuitApp => {
                            gui_app_clone.quit(); // GTKアプリケーションを終了
                            return glib::ControlFlow::Break; // イベントループを終了
                        }
                    }
                    glib::ControlFlow::Continue // 継続してアイドルイベントを処理
                });
            }
        }
        Err(e) => {
            // バインドに失敗した場合（ポートが使用中）、別のインスタンスが既に実行中と判断します。
            println!("GTK: Failed to bind to port {} ({:?}). Another instance might be running.", IPC_PORT, e);
            println!("GTK: This is a secondary instance. Sending command to primary instance.");

            // このインスタンスはクライアントとして動作し、初期コマンドを既存のインスタンスに送信して終了します。
            if let Some(cmd) = initial_command {
                if let Err(send_err) = send_ipc_command(cmd) {
                    eprintln!("GTK: Failed to send command to primary instance: {}", send_err);
                }
            } else {
                println!("GTK: No initial command provided for secondary instance. Exiting.");
            }
            // セカンダリインスタンスはGTKアプリを起動せず、すぐに終了します。
            return Ok(());
        }
    }

    // --------------------------------------------------------------------
    // 2. GTKアプリケーションのオープンハンドラを設定
    // --------------------------------------------------------------------
    // GTKアプリケーションがアクティブ化されたとき（例: 初回起動時や `activate()` が呼ばれたとき）に
    // 実行されるロジックを定義します。
    gui_app.connect_open(move |app, _f, _hint| {
        use gui::Button;
        use gui::prelude::*;

        let button = Button::builder()
            .label("Press me!")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        // ボタンの"clicked"シグナルに接続します。
        button.connect_clicked(move |_| {
            // GUIは5秒間ブロックされます（これは例であり、実際のアプリケーションでは避けるべきです）。
            // 実際のアプリケーションでは、長時間かかる処理は別のスレッドで行うべきです。
            let five_seconds = Duration::from_secs(5);
            thread::sleep(five_seconds);
            println!("Button clicked and GUI unblocked!");
        });

        let window = App::window_builder(app, "Hello, World")
            .child(&button)
            .width_request(480)
            .height_request(360)
            .build();

        // ウィンドウがアクティブ化されたときのデフォルトの動作を設定します（ここでは何もしません）。
        window.connect_activate_default(|_window| {});
        // ウィンドウを表示します。
        window.present();
    });

    // --------------------------------------------------------------------
    // 3. GTKのメインループで内部チャネルからのメッセージを処理
    // --------------------------------------------------------------------
    // `glib::idle_add_local` は、GTKのメインループがアイドル状態になったときに実行されるクロージャをスケジュールします。
    // これにより、IPCサーバーから`tx_ipc`経由で送られてきたメッセージをGTKメインスレッドで安全に処理できます。
    let gui_app_clone_for_idle = gui_app.clone(); // idleクロージャ内で使用するためにクローン
    glib::idle_add_local(move || {
        // ノンブロッキングでメッセージを試行受信します。
        // `try_recv()` は、メッセージがあれば `Ok(T)` を返し、なければ `Err(Empty)` を返します。
        match rx_ipc.try_recv() {
            Ok(TrayMessage::OpenWindow) => {
                println!("GTK: Received OpenWindow message from IPC. Activating app.");
                gui_app_clone_for_idle.activate(); // アプリケーションをアクティブ化し、ウィンドウを表示
            }
            Ok(TrayMessage::QuitApp) => {
                println!("GTK: Received QuitApp message from IPC. Quitting app.");
                gui_app_clone_for_idle.quit(); // GTKアプリケーションを終了
                return glib::ControlFlow::Break; // イベントループを終了
            }
            Err(mpsc::TryRecvError::Empty) => {
                // メッセージがない場合は何もしません。
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                // 送信側が切断された場合（IPCサーバーが予期せず終了した場合など）
                eprintln!("GTK: IPC channel disconnected. Quitting app.");
                gui_app_clone_for_idle.quit(); // ここで quit を呼ぶのは、IPCサーバーが落ちた場合に備えて
                return glib::ControlFlow::Break; // イベントループを終了
            }
        }
        glib::ControlFlow::Continue // 継続してアイドルイベントを処理するよう指示
    });

    // --------------------------------------------------------------------
    // 4. メインスレッドでGTKイベントループを実行
    // --------------------------------------------------------------------
    // ここでGTKアプリケーションのメインイベントループが開始され、UIイベントやアイドルイベントを処理します。
    println!("Starting GTK application event loop.");
    gui_app.run();
    println!("GTK application event loop finished.");

    Ok(())
}

/// IPCサーバーのスレッド関数
/// TCPリスナーからの接続を待ち受け、受信したコマンドをGTKメインスレッドに転送します。
///
/// `listener`: クライアントからの接続を待ち受けるTCPリスナー。
/// `tx`: 受信したメッセージをGTKメインスレッドに送信するためのSender。
/// `gui_app`: GTKアプリケーションインスタンスのクローン。エラー時にアプリケーションを終了するために使用。
fn ipc_server_thread(listener: TcpListener, tx: Sender<TrayMessage>) { // gui_app パラメータを削除
    // リスナーからの接続を無限ループで待ち受けます。
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("IPC Server: New client connected.");
                let mut buffer = [0; 512]; // メッセージを受信するためのバッファ

                // クライアントからのメッセージを読み込みます。
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        // 受信したバイト列をUTF-8文字列に変換します。
                        let msg = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println!("IPC Server: Received message: '{}'", msg.trim());

                        // 受信した文字列メッセージを `TrayMessage` Enumに変換します。
                        let tray_msg = match msg.trim() {
                            "OPEN" => Some(TrayMessage::OpenWindow),
                            "QUIT" => Some(TrayMessage::QuitApp),
                            _ => {
                                eprintln!("IPC Server: Unknown command received: {}", msg.trim());
                                None
                            }
                        };

                        // 変換されたメッセージがあれば、GTKメインスレッドに送信します。
                        if let Some(tm) = tray_msg {
                            if let Err(e) = tx.send(tm) {
                                eprintln!("IPC Server: Failed to send message to GTK main thread: {}", e);
                                // メインスレッドへの送信が失敗した場合、それはメインGTKアプリがすでに終了しているか、
                                // 終了中であることを意味します。このスレッドも終了して構いません。
                                break; // サーバーを終了します。
                            }
                        }
                    }
                    Err(e) => eprintln!("IPC Server: Failed to read from stream: {}", e),
                }
            }
            Err(e) => {
                eprintln!("IPC Server: Failed to accept connection: {}", e);
                break; // エラーが発生したらサーバーを終了します。
            }
        }
    }
    println!("IPC Server thread finished.");
}

/// IPCクライアントとしてコマンドを既存のプライマリインスタンスに送信します。
///
/// `command`: 送信する `TrayMessage`。
///
/// 成功した場合は `Ok(())`、失敗した場合は `std::io::Error` を返します。
pub fn send_ipc_command(command: TrayMessage) -> Result<(), io::Error> {
    let cmd_str = match command {
        TrayMessage::OpenWindow => "OPEN",
        TrayMessage::QuitApp => "QUIT",
    };

    println!("IPC Client: Attempting to send '{}' to primary instance on port {}.", cmd_str, IPC_PORT);

    // 指定されたポートで実行中のサーバーに接続を試みます。
    match TcpStream::connect(format!("127.0.0.1:{}", IPC_PORT)) {
        Ok(mut stream) => {
            // コマンド文字列をストリームに書き込み、改行で終端します。
            stream.write_all(format!("{}\n", cmd_str).as_bytes())?;
            println!("IPC Client: Command sent successfully.");
            Ok(())
        }
        Err(e) => {
            eprintln!("IPC Client: Could not connect to primary instance: {}", e);
            Err(e)
        }
    }
}
