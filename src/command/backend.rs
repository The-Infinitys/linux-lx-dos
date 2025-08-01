use crate::LxDosError;
use crate::modules::app::gui::Gui;
use crate::modules::app::instance::{InstanceMessage, WindowClient, WindowType};
use async_channel::{self, Receiver, Sender};
use gui::glib::{self, MainContext};
use instance_pipe::Client;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub fn run_backend(pipe_name: &str) -> Result<(), LxDosError> {
    let gui = Gui::new();
    let client = Client::start(pipe_name)?;
    let pipe_name = pipe_name.to_string();
    let window_client = Arc::new(WindowClient::new(client));
    let client_handle = Arc::new(Mutex::new(None::<JoinHandle<Result<(), LxDosError>>>));

    let window_client_clone_gui_handler = Arc::clone(&window_client);
    let client_handle_clone_gui_handler = Arc::clone(&client_handle);
    let pipe_name_clone_gui_handler = pipe_name.clone();

    gui.handler(move |app: &gui::Application| {
        use gui::prelude::*;

        let app_clone = app.clone();
        let (tx, rx): (Sender<InstanceMessage>, Receiver<InstanceMessage>) =
            async_channel::unbounded();

        let window_client_thread_clone = Arc::clone(&window_client_clone_gui_handler);
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
        *client_handle_clone_gui_handler.lock().unwrap() = Some(handle);

        let main_context = MainContext::default();
        let app_clone_for_idle = app_clone.clone();
        let rx = Arc::new(Mutex::new(rx));

        let window_client_clone_idle = Arc::clone(&window_client_clone_gui_handler);
        main_context
            .with_thread_default(|| {
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
                                    let window_title = match window_type {
                                        WindowType::Main => "Lx-DOS Main",
                                        WindowType::Settings => "Lx-DOS Settings",
                                    };

                                    let button = gui::Button::builder()
                                        .label("Press me!")
                                        .margin_top(12)
                                        .margin_bottom(12)
                                        .margin_start(12)
                                        .margin_end(12)
                                        .build();

                                    let window =
                                        Gui::window_builder(&app_clone_for_idle, window_title)
                                            .child(&button)
                                            .width_request(480)
                                            .height_request(360)
                                            .build();

                                    let window_weak = window.downgrade();
                                    button.connect_clicked(move |_| {
                                        if let Some(window) = window_weak.upgrade() {
                                            println!("Button clicked, closing window");
                                            window.close();
                                        }
                                    });

                                    let window_client_clone_close_request =
                                        Arc::clone(&window_client_clone_idle);
                                    let pipe_name_clone_close_request = pipe_name.clone();
                                    window.connect_close_request(move |window| {
                                        println!(
                                            "Window close requested, sending CloseWindow: {}",
                                            pipe_name_clone_close_request
                                        );
                                        if let Err(e) = window_client_clone_close_request.send(
                                            &InstanceMessage::CloseWindow {
                                                pipe_name: pipe_name_clone_close_request.clone(),
                                            },
                                        ) {
                                            eprintln!(
                                                "Failed to send CloseWindow on window close: {}",
                                                e
                                            );
                                        }
                                        window.close();
                                        glib::Propagation::Proceed
                                    });

                                    window.present();
                                }
                                InstanceMessage::CloseWindow { pipe_name } => {
                                    println!("Received CloseWindow for pipe: {}", pipe_name);
                                    app_clone_for_idle.quit();
                                }
                                InstanceMessage::MaximizeWindow { pipe_name } => {
                                    println!("Received MaximizeWindow for pipe: {}", pipe_name);
                                    if let Some(window) = app_clone_for_idle.active_window() {
                                        window.maximize();
                                    }
                                }
                                InstanceMessage::MinimizeWindow { pipe_name } => {
                                    println!("Received MinimizeWindow for pipe: {}", pipe_name);
                                    if let Some(window) = app_clone_for_idle.active_window() {
                                        window.minimize();
                                    }
                                }
                                InstanceMessage::RestoreWindow { pipe_name } => {
                                    println!("Received RestoreWindow for pipe: {}", pipe_name);
                                    if let Some(window) = app_clone_for_idle.active_window() {
                                        window.unmaximize();
                                        window.present();
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
            })
            .expect("Failed to attach idle source to MainContext");

        app_clone.connect_window_added(move |_, window| {
            println!("Window added to application");
            window.present();
        });

        app_clone.connect_window_removed(move |app, _| {
            println!("Window removed from application");
            app.quit();
        });
        let pipe_name_clone_bg = pipe_name_clone_gui_handler.clone();
        thread::spawn(move || {
            println!("Background thread started for pipe: {}", pipe_name_clone_bg);
            thread::sleep(Duration::from_secs(1));
            println!(
                "Background thread task completed for pipe: {}",
                pipe_name_clone_bg
            );
            Ok::<(), LxDosError>(())
        });
    });
    gui.run();

    if let Some(handle) = client_handle.lock().unwrap().take() {
        handle
            .join()
            .map_err(|e| LxDosError::Message(format!("Client thread panicked: {:?}", e)))??;
    }
    Ok(())
}
