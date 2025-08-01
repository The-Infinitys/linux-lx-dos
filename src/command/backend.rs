use crate::LxDosError;
use crate::modules::app::gui::Gui;
use crate::modules::app::instance::{InstanceMessage, WindowClient, WindowType};
use gui::glib::{self, MainContext};
use instance_pipe::Client;
use async_channel::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub fn run_backend(pipe_name: &str) -> Result<(), LxDosError> {
    let gui = Gui::new();
    let client = Client::start(pipe_name)?;
    let pipe_name = pipe_name.to_string();
    let window_client = Arc::new(WindowClient::new(client));
    let client_handle = Arc::new(Mutex::new(None::<JoinHandle<Result<(), LxDosError>>>));

    let pipe_name_clone = pipe_name.clone();
    let window_client_clone = Arc::clone(&window_client);
    let client_handle_clone = Arc::clone(&client_handle);
    gui.handler(move |app: &gui::Application| {
        use gui::prelude::*;
        // appをArc<Mutex<gtk4::Application>>でラップ
        let app = Arc::new(Mutex::new(app.clone()));

        // async_channelを作成
        let (tx, rx): (Sender<InstanceMessage>, Receiver<InstanceMessage>) = async_channel::unbounded();

        // クライアントポーリングスレッドの初期化
        let window_client_thread_clone = Arc::clone(&window_client_clone);
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            loop {
                match window_client_thread_clone.poll_event() {
                    Ok(messages) => {
                        for message in messages {
                            println!("Sending message to channel: {:?}", message);
                            if let Err(e) = tx_clone.send_blocking(message) {
                                eprintln!("Failed to send message to channel: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Client poll error: {}", e);
                        break;
                    }
                }
                thread::sleep(Duration::from_millis(50));
            }
            Ok::<(), LxDosError>(())
        });
        *client_handle_clone.lock().unwrap() = Some(handle);

        // チャネル受信をメインループに統合
        let main_context = MainContext::default();
        let app_clone = Arc::clone(&app);
        let rx = Arc::new(Mutex::new(rx));
        main_context.with_thread_default(|| {
            glib::source::idle_add_local(move || {
                let rx = rx.lock().unwrap();
                match rx.try_recv() {
                    Ok(message) => {
                        match message {
                            InstanceMessage::OpenWindow {
                                pipe_name,
                                window_type,
                            } => {
                                println!(
                                    "Received OpenWindow for pipe: {}, type: {:?}",
                                    pipe_name, window_type
                                );
                            }
                            InstanceMessage::CloseWindow { pipe_name } => {
                                println!("Received CloseWindow for pipe: {}", pipe_name);
                                if let Ok(app) = app_clone.lock() {
                                    app.quit();
                                }
                            }
                            InstanceMessage::MaximizeWindow { pipe_name } => {
                                println!("Received MaximizeWindow for pipe: {}", pipe_name);
                                if let Ok(app) = app_clone.lock() {
                                    if let Some(window) = app.active_window() {
                                        window.maximize();
                                    }
                                }
                            }
                            InstanceMessage::MinimizeWindow { pipe_name } => {
                                println!("Received MinimizeWindow for pipe: {}", pipe_name);
                                if let Ok(app) = app_clone.lock() {
                                    if let Some(window) = app.active_window() {
                                        window.minimize();
                                    }
                                }
                            }
                            InstanceMessage::RestoreWindow { pipe_name } => {
                                println!("Received RestoreWindow for pipe: {}", pipe_name);
                                if let Ok(app) = app_clone.lock() {
                                    if let Some(window) = app.active_window() {
                                        window.unmaximize();
                                        window.present();
                                    }
                                }
                            }
                        }
                        glib::ControlFlow::Continue
                    }
                    Err(async_channel::TryRecvError::Empty) => glib::ControlFlow::Continue,
                    Err(async_channel::TryRecvError::Closed) => {
                        println!("Channel closed, stopping receiver");
                        glib::ControlFlow::Break
                    }
                }
            });
        }).expect("Failed to attach idle source to MainContext");

        // アプリケーションのactivateシグナルの接続
        let pipe_name_clone_for_activate = pipe_name_clone.clone();
        let window_client_clone_for_activate = Arc::clone(&window_client_clone);
        app.lock().unwrap().connect_activate(move |app| {
            // ボタンの作成
            let button = gui::Button::builder()
                .label("Press me!")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();

            let pipe_name_clone = pipe_name_clone_for_activate.clone();
            let window_client_clone = Arc::clone(&window_client_clone_for_activate);
            button.connect_clicked(move |_| {
                println!("Button clicked, sending CloseWindow: {}", pipe_name_clone);
                if let Err(e) = window_client_clone.send(&InstanceMessage::CloseWindow {
                    pipe_name: pipe_name_clone.clone(),
                }) {
                    eprintln!("Failed to send CloseWindow: {}", e);
                }
            });

            // ウィンドウタイトルの設定
            let window_title = match window_client_clone_for_activate.poll_event() {
                Ok(messages) => messages
                    .iter()
                    .find_map(|msg| {
                        if let InstanceMessage::OpenWindow { window_type, .. } = msg {
                            Some(match window_type {
                                WindowType::Main => "Lx-DOS Main",
                                WindowType::Settings => "Lx-DOS Settings",
                            })
                        } else {
                            None
                        }
                    })
                    .unwrap_or("Lx-DOS Window"),
                Err(_) => "Lx-DOS Window",
            };

            // ウィンドウの作成と設定
            let window = Gui::window_builder(app, window_title)
                .child(&button)
                .width_request(480)
                .height_request(360)
                .build();

            // ウィンドウの閉じるイベントを処理
            let window_client_clone = Arc::clone(&window_client_clone_for_activate);
            let pipe_name_clone = pipe_name_clone_for_activate.clone();
            window.connect_close_request(move |_| {
                println!("Window closed, sending CloseWindow: {}", pipe_name_clone);
                if let Err(e) = window_client_clone.send(&InstanceMessage::CloseWindow {
                    pipe_name: pipe_name_clone.clone(),
                }) {
                    eprintln!("Failed to send CloseWindow on window close: {}", e);
                }
                glib::Propagation::Stop
            });

            window.present();
        });

        // window-addedシグナルの接続
        app.lock().unwrap().connect_window_added(move |_, window| {
            println!("Window added to application");
            window.present();
        });

        // window-removedシグナルの接続
        app.lock().unwrap().connect_window_removed(move |app, _| {
            println!("Window removed from application");
            if app.windows().is_empty() {
                app.quit();
            }
        });

        // バックグラウンドスレッドの例
        let window_client_clone = Arc::clone(&window_client_clone);
        let pipe_name_clone = pipe_name_clone.clone();
        thread::spawn(move || {
            // window_client_cloneを使用した例
            if let Err(e) = window_client_clone.send(&InstanceMessage::OpenWindow {
                pipe_name: pipe_name_clone.clone(),
                window_type: WindowType::Main,
            }) {
                eprintln!("Failed to send OpenWindow in background thread: {}", e);
            }
            println!("Background thread started for pipe: {}", pipe_name_clone);
            thread::sleep(Duration::from_secs(1));
            println!(
                "Background thread task completed for pipe: {}",
                pipe_name_clone
            );
            Ok::<(), LxDosError>(())
        });
    });

    gui.run();

    // クライアントスレッドハンドルの結合
    if let Some(handle) = client_handle.lock().unwrap().take() {
        handle
            .join()
            .map_err(|e| LxDosError::Message(format!("Client thread panicked: {:?}", e)))??;
    }

    Ok(())
}