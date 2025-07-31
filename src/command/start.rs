// src/start.rs

use crate::LxDosError;
use crate::command::run::send_ipc_command; // runモジュールからIPC関連の関数と定数をインポート
use crate::modules::app::App;
use crate::modules::app::messages::TrayMessage; // 共有メッセージ型をインポート
use std::process::Command; // 新しいプロセスを起動するためにインポート
use std::thread; // スレッド操作をインポート
use system_tray::{Event as SystemTrayEvent, Menu as SystemTrayMenu};

/// アプリケーションのエントリーポイント
///
/// この関数はシステムトレイのロジックを管理し、GUIアプリケーションプロセスとのIPCを調整します。
pub fn start() -> Result<(), LxDosError> {
    // システムトレイのスレッドを開始します。
    // このスレッドは、システムトレイのイベントをポーリングし、それに応じてIPCメッセージを送信するか、
    // 新しいGUIアプリケーションプロセスを起動します。
    let tray_thread_handle = thread::spawn(move || {
        if let Err(e) = start_system_tray_thread() {
            eprintln!("System tray thread encountered an error: {}", e);
        }
    });

    // システムトレイのスレッドが終了するのを待ちます。
    // 通常、これはアプリケーションが終了するまでブロックされます。
    if let Err(e) = tray_thread_handle.join() {
        eprintln!("Failed to join system tray thread: {:?}", e);
    }

    Ok(()) // `start()` 関数自体はGUIアプリを実行しなくなりました。
}

/// システムトレイを初期化し、イベントをポーリングするスレッドを開始します。
///
/// このスレッドは、システムトレイのメニュー項目がクリックされたときに、
/// 既存のGUIアプリケーションにIPCメッセージを送信するか、新しいGUIアプリケーションプロセスを起動します。
fn start_system_tray_thread() -> Result<(), LxDosError> {
    // システムトレイのアイコンとメニューを設定します。
    let tray = App::system_tray()
        .icon(include_bytes!("../../public/icon.svg"), "svg") // アイコンファイルと形式を指定
        .menu(SystemTrayMenu::new("Open".to_string(), "open".to_string())) // "Open" メニュー項目
        .menu(SystemTrayMenu::new("Quit".to_string(), "quit".to_string())); // "Quit" メニュー項目

    // システムトレイを開始し、アイコンとメニューを表示します。
    tray.start();

    println!("System tray started.");

    // システムトレイのイベントをポーリングし、IPC経由でメッセージを送信するか、プロセスを起動します。
    loop {
        match tray.poll_event() {
            Ok(SystemTrayEvent::MenuItemClicked(id)) => {
                // クリックされたメニュー項目のIDに基づいて、送信するコマンドを決定します。
                let command_to_send = match id.as_str() {
                    "open" => Some(TrayMessage::OpenWindow),
                    "quit" => Some(TrayMessage::QuitApp),
                    _ => None, // 不明なIDは無視
                };

                if let Some(cmd) = command_to_send {
                    println!(
                        "System tray: '{}' clicked. Attempting to send IPC message or launch app.",
                        id
                    );

                    // まず、既存のGUIアプリにIPCメッセージを送信しようと試みます。
                    match send_ipc_command(cmd) {
                        Ok(_) => {
                            println!("System tray: IPC message sent successfully.");
                            // Quitメッセージが送信された場合、システムトレイのスレッドも終了します。
                            if matches!(cmd, TrayMessage::QuitApp) {
                                break;
                            }
                        }
                        // 接続拒否またはアドレス利用不可の場合、GUIアプリが実行されていないと判断します。
                        Err(e)
                            if e.kind() == std::io::ErrorKind::ConnectionRefused
                                || e.kind() == std::io::ErrorKind::AddrNotAvailable =>
                        {
                            println!(
                                "System tray: GUI app not running, launching new process with command: {:?}",
                                cmd
                            );
                            // 新しいGUIアプリプロセスを起動します。
                            // ここで `target/debug/gui-app-run` は、`run.rs`がコンパイルされたバイナリのパスです。
                            // ご自身のプロジェクトの構造に合わせてパスを調整してください。
                            let mut command = Command::new("target/debug/gui-app-run");
                            command.arg("--command");
                            command.arg(match cmd {
                                TrayMessage::OpenWindow => "open",
                                TrayMessage::QuitApp => "quit",
                            });

                            match command.spawn() {
                                Ok(_) => println!("System tray: GUI app process launched."),
                                Err(spawn_err) => eprintln!(
                                    "System tray: Failed to launch GUI app process: {}",
                                    spawn_err
                                ),
                            }

                            // Quitコマンドを送信（または起動）したら、システムトレイのスレッドも終了します。
                            if matches!(cmd, TrayMessage::QuitApp) {
                                break;
                            }
                        }
                        Err(e) => {
                            // その他のIPC送信エラーを処理します。
                            eprintln!("System tray: Error sending IPC message: {}", e);
                            if matches!(cmd, TrayMessage::QuitApp) {
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                // システムトレイイベントのポーリング中にエラーが発生した場合。
                eprintln!("Error polling system tray event: {}", e);
                break; // エラーが発生したらループを終了します。
            }
            _ => {
                // その他のイベントは無視します。必要に応じてここに処理を追加できます。
            }
        }
        // ポーリング頻度を調整するために短いスリープを入れることもできます。
        // thread::sleep(Duration::from_millis(100));
    }
    println!("System tray thread finished.");
    Ok(())
}
