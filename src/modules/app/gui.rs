use super::App;
use async_channel::{Receiver, Sender};
use gui::builders::ApplicationWindowBuilder;
use gui::gio::prelude::ApplicationExtManual;
use gui::glib::{self, MainContext};
use home::home_dir;
use instance_pipe::Client;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use crate::modules::app::instance::InstanceMessage;
use crate::LxDosError;

// GUIアプリケーションのメイン構造体
pub struct Gui {
    gui: gui::Application,
    message_sender: Option<Sender<InstanceMessage>>,
    message_receiver: Option<Receiver<InstanceMessage>>,
    client_handle: Arc<Mutex<Option<JoinHandle<Result<(), LxDosError>>>>>,
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

        Self::install_icons();

        let gui = gui::Application::builder()
            .application_id(App::app_id())
            .flags(flags)
            .build();

        Self {
            gui,
            message_sender: None,
            message_receiver: None,
            client_handle: Arc::new(Mutex::new(None)),
        }
    }

    /// Freedesktopの仕様に基づいて、ユーザーのローカルディレクトリにアイコンをインストールします。
    fn install_icons() {
        let home_dir = match home_dir() {
            Some(path) => path,
            None => {
                eprintln!("Home directory not found. Icon will not be installed.");
                return;
            }
        };

        let icon_name = Self::icon_name();
        
        let bin_data = [
            (16, include_bytes!(concat!(env!("OUT_DIR"), "/16x16.png")).to_vec()),
            (24, include_bytes!(concat!(env!("OUT_DIR"), "/24x24.png")).to_vec()),
            (32, include_bytes!(concat!(env!("OUT_DIR"), "/32x32.png")).to_vec()),
            (48, include_bytes!(concat!(env!("OUT_DIR"), "/48x48.png")).to_vec()),
            (64, include_bytes!(concat!(env!("OUT_DIR"), "/64x64.png")).to_vec()),
            (128, include_bytes!(concat!(env!("OUT_DIR"), "/128x128.png")).to_vec()),
            (256, include_bytes!(concat!(env!("OUT_DIR"), "/256x256.png")).to_vec()),
            (512, include_bytes!(concat!(env!("OUT_DIR"), "/512x512.png")).to_vec()),
        ];
        
        for (size, bytes) in bin_data.iter() {
            let size_dir_name = format!("{}x{}", size, size);
            let icon_path = home_dir.join(format!(".local/share/icons/hicolor/{}/apps/", size_dir_name));
            
            if let Err(e) = fs::create_dir_all(&icon_path) {
                eprintln!("Failed to create icon directory: {}", e);
                continue;
            }
            
            let icon_file_path = icon_path.join(format!("{}.png", icon_name));

            if icon_file_path.exists() {
                println!(
                    "Icon file for size {} already exists at {}. Skipping installation.",
                    size,
                    icon_file_path.display()
                );
                continue;
            }

            let mut file = match File::create(&icon_file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!(
                        "Failed to create icon file at {}: {}",
                        icon_file_path.display(),
                        e
                    );
                    continue;
                }
            };
    
            if let Err(e) = file.write_all(bytes) {
                eprintln!(
                    "Failed to write to icon file at {}: {}",
                    icon_file_path.display(),
                    e
                );
            } else {
                println!("Icon successfully installed at {}.", icon_file_path.display());
            }
        }
        
        let svg_path = home_dir.join(".local/share/icons/hicolor/scalable/apps/");
        if let Err(e) = fs::create_dir_all(&svg_path) {
            eprintln!("Failed to create SVG icon directory: {}", e);
            return;
        }
        let svg_file_path = svg_path.join(format!("{}.svg", icon_name));

        if !svg_file_path.exists() {
            let svg_bytes = include_bytes!("../../../public/icon.svg");
            let mut file = match File::create(&svg_file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to create SVG file at {}: {}", svg_file_path.display(), e);
                    return;
                }
            };
            if let Err(e) = file.write_all(svg_bytes) {
                eprintln!("Failed to write to SVG file at {}: {}", svg_file_path.display(), e);
            } else {
                println!("SVG icon successfully installed at {}.", svg_file_path.display());
            }
        } else {
            println!("SVG icon file already exists at {}. Skipping installation.", svg_file_path.display());
        }
    }

    fn icon_name() -> String {
        format!("{}-app-icon", App::app_id().to_ascii_lowercase())
    }

    pub fn get_icon_cache_files() -> Vec<PathBuf> {
        let home_dir = match home_dir() {
            Some(path) => path,
            None => {
                eprintln!("Home directory not found.");
                return Vec::new();
            }
        };

        let icon_name = Self::icon_name();
        let icon_sizes = [16, 24, 32, 48, 64, 128, 256, 512];
        let mut paths: Vec<PathBuf> = Vec::new();

        for size in icon_sizes.iter() {
            let size_dir_name = format!("{}x{}", size, size);
            let icon_path = home_dir.join(format!(
                ".local/share/icons/hicolor/{}/apps/{}.png",
                size_dir_name, icon_name
            ));
            if icon_path.exists() {
                paths.push(icon_path);
            }
        }

        let svg_icon_path = home_dir.join(format!(
            ".local/share/icons/hicolor/scalable/apps/{}.svg",
            icon_name
        ));
        if svg_icon_path.exists() {
            paths.push(svg_icon_path);
        }
        paths
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

    /// バックエンドからのメッセージを受信し、GUIスレッドで処理します。
    pub fn on_message<F>(&self, f: F)
    where
        F: Fn(InstanceMessage) + 'static + Send,
    {
        let rx = self
            .message_receiver
            .as_ref()
            .expect("Message receiver not initialized.");

        let rx_clone = Arc::new(Mutex::new(rx.clone()));

        MainContext::default()
            .with_thread_default(move || {
                glib::source::idle_add_local(move || {
                    let rx = rx_clone.lock().unwrap();
                    match rx.try_recv() {
                        Ok(message) => {
                            f(message);
                            glib::ControlFlow::Continue
                        }
                        Err(async_channel::TryRecvError::Empty) => glib::ControlFlow::Continue,
                        Err(async_channel::TryRecvError::Closed) => {
                            println!("Channel closed, stopping receiver");
                            glib::ControlFlow::Break
                        }
                    }
                })
            })
            .expect("Error happened while glib running");
    }

    /// GUIアプリケーションのイベントハンドラを設定します。
    pub fn handler<F: Fn(&gui::Application, &Sender<InstanceMessage>) + 'static>(&mut self, f: F) {
        let (tx, rx) = async_channel::unbounded();
        self.message_sender = Some(tx);
        self.message_receiver = Some(rx);

        let tx_clone = self.message_sender.clone().unwrap();
        self.gui.connect_open(move |app, _files, _hint| {
            f(app, &tx_clone);
        });
    }

    /// GUIアプリケーションを実行し、バックエンドとの通信スレッドを管理します。
    pub fn run(&mut self, pipe_name: &str) {
        let pipe_name = pipe_name.to_string();
        let client_handle_clone: Arc<Mutex<Option<JoinHandle<Result<(), LxDosError>>>>> =
            Arc::clone(&self.client_handle);
        let sender_for_thread = self.message_sender.clone().unwrap();

        // バックエンドとの通信スレッドを起動
        let handle = thread::spawn(move || {
            let mut client = Client::start(&pipe_name)?;
            loop {
                // poll_eventはVec<InstanceMessage>を直接返す
                match client.poll_event::<InstanceMessage>() {
                    Ok(messages) => {
                        if let Some(message) = messages {
                            if let instance_pipe::Event::MessageReceived(msg) = message {
                                if let Err(e) = sender_for_thread.send_blocking(msg) {
                                    eprintln!("Failed to send message to channel: {}", e);
                                    break;
                                }
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

        self.gui.run();

        // アプリケーション終了時にスレッドをクリーンアップ
        if let Some(handle) = self.client_handle.lock().unwrap().take() {
            if let Err(e) = handle.join() {
                eprintln!("Client thread panicked: {:?}", e);
            }
        }
        println!("Application closed");
    }
}