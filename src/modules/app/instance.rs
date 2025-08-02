use crate::LxDosError;
use instance_pipe::{Client, Event, Server};
use std::collections::HashMap;
use std::env;
use std::ops::DerefMut;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Hash)]
pub enum WindowType {
    Main,
    Settings,
}

impl std::fmt::Display for WindowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum InstanceMessage {
    OpenWindow {
        pipe_name: String,
        window_type: WindowType,
    },
    CloseWindow {
        pipe_name: String,
    },
    MaximizeWindow {
        pipe_name: String,
    },
    MinimizeWindow {
        pipe_name: String,
    },
    RestoreWindow {
        pipe_name: String,
    },
}

#[derive(Clone)]
pub struct WindowServer {
    server: Arc<Mutex<Server>>,
    child: Arc<Mutex<Option<Child>>>,
    clients: Arc<Mutex<Vec<Client>>>,
}

impl WindowServer {
    pub fn new(server: Server, child: Option<Child>) -> Self {
        Self {
            server: Arc::new(Mutex::new(server)),
            child: Arc::new(Mutex::new(child)),
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn poll_event(&self) -> Result<Vec<InstanceMessage>, LxDosError> {
        let mut server = self
            .server
            .lock()
            .map_err(|e| LxDosError::Message(e.to_string()))?;
        let mut messages = Vec::new();

        match server.poll_event() {
            Ok(Some(Event::ConnectionAccepted(client))) => {
                println!("New client connected to server");
                let mut clients = self
                    .clients
                    .lock()
                    .map_err(|e| LxDosError::Message(e.to_string()))?;
                clients.push(client.clone());
            }
            Ok(Some(Event::MessageReceived(_))) => {
                println!("Unexpected MessageReceived event in server");
            }
            Ok(Some(Event::MessageSent)) => {
                println!("Server sent a message");
            }
            Ok(None) => {}
            Err(e) => return Err(LxDosError::Io(e)),
        }

        let mut clients = self
            .clients
            .lock()
            .map_err(|e| LxDosError::Message(e.to_string()))?;
        let mut i = 0;
        while i < clients.len() {
            match clients[i].poll_event::<InstanceMessage>() {
                Ok(Some(Event::MessageReceived(message))) => {
                    messages.push(message);
                    i += 1;
                }
                Ok(Some(Event::MessageSent)) => {
                    println!("Client sent a message");
                    i += 1;
                }
                Ok(Some(Event::ConnectionAccepted(_))) => {
                    println!("Unexpected connection event in client");
                    i += 1;
                }
                Ok(None) => {
                    i += 1;
                }
                Err(e) => {
                    println!("Removing disconnected client: {}", e);
                    clients.remove(i);
                }
            }
        }

        Ok(messages)
    }

    // 子プロセスが終了したかをチェックする新しいメソッド
    pub fn check_child_status(&self) -> Result<bool, LxDosError> {
        if let Ok(mut child_lock) = self.child.lock() {
            if let Some(child) = child_lock.as_mut() {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        println!("Child process exited with status: {}", status);
                        Ok(true)
                    }
                    Ok(None) => Ok(false), // まだ実行中
                    Err(e) => Err(LxDosError::Io(e)),
                }
            } else {
                Ok(false) // Child process handle doesn't exist
            }
        } else {
            Err(LxDosError::Message(
                "Failed to lock child mutex".to_string(),
            ))
        }
    }
}

impl Drop for WindowServer {
    fn drop(&mut self) {
        if let Ok(mut child) = self.child.lock() {
            if let Some(mut child) = child.take() {
                if let Err(e) = child.kill() {
                    log::error!("Failed to kill child process: {}", e);
                }
                if let Err(e) = child.wait() {
                    log::error!("Failed to wait for child process: {}", e);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct WindowClient {
    client: Arc<Mutex<Client>>,
}

impl WindowClient {
    pub fn new(client: Client) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }

    pub fn send(&self, message: &InstanceMessage) -> Result<(), LxDosError> {
        let client = self
            .client
            .lock()
            .map_err(|e| LxDosError::Message(e.to_string()))?;
        client.send(message)?;
        Ok(())
    }

    pub fn poll_event(&self) -> Result<Vec<InstanceMessage>, LxDosError> {
        let mut client = self
            .client
            .lock()
            .map_err(|e| LxDosError::Message(e.to_string()))?;
        let mut messages = Vec::new();
        match client.poll_event::<InstanceMessage>() {
            Ok(Some(Event::MessageReceived(message))) => {
                messages.push(message);
                Ok(messages)
            }
            Ok(Some(Event::ConnectionAccepted(_))) => {
                println!("Unexpected connection event in client");
                Ok(messages)
            }
            Ok(Some(Event::MessageSent)) => {
                println!("Client sent a message");
                Ok(messages)
            }
            Ok(None) => Ok(messages),
            Err(e) => Err(LxDosError::Io(e)),
        }
    }
}

pub struct Window {
    pub pipe_name: String,
    pub server: WindowServer,
    pub client: WindowClient,
}

pub struct WindowManager {
    pipe_name: String,
    windows: HashMap<WindowType, Window>,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowManager {
    pub fn new() -> Self {
        let pipe_name = format!("lxdos_pipe_{}", std::process::id());
        Self {
            pipe_name,
            windows: HashMap::new(),
        }
    }

    pub fn poll_event(&mut self) -> Result<Vec<InstanceMessage>, LxDosError> {
        let mut messages = Vec::new();
        let mut windows_to_close = Vec::new();

        // 終了した子プロセスや切断されたパイプを検知
        for (window_type, window) in &self.windows {
            match window.server.poll_event() {
                Ok(mut new_messages) => {
                    for message in &new_messages {
                        if let InstanceMessage::CloseWindow { .. } = message {
                            windows_to_close.push(window_type.clone());
                        }
                    }
                    messages.append(&mut new_messages);
                }
                Err(e) => {
                    println!("Server for {:?} disconnected: {}", window_type, e);
                    windows_to_close.push(window_type.clone());
                }
            }

            if window.server.check_child_status()? {
                println!("Child process for {:?} exited.", window_type);
                windows_to_close.push(window_type.clone());
            }
        }

        // CloseWindowメッセージに基づいてウィンドウを削除
        for closed_window_type in windows_to_close {
            println!(
                "Cleaning up resources for closed window: {:?}",
                closed_window_type
            );
            self.windows.remove(&closed_window_type);
        }

        Ok(messages)
    }
    // WindowManager::open_window メソッドの修正
    pub fn open_window(&mut self, window_type: WindowType) -> Result<(), LxDosError> {
        if self.windows.contains_key(&window_type) {
            println!("Window of type {:?} is already open", window_type);
            return Ok(());
        }

        let current_exe = env::current_exe()?;
        let pid = std::process::id().to_string();
        let child_pipe_name = format!(
            "{}_{}",
            self.pipe_name,
            window_type.to_string().to_ascii_lowercase()
        );

        // 子プロセスを起動する際に、子プロセス用のパイプ名を引数として渡す
        let child = Command::new(current_exe)
            .env("LXDOS_BACKEND", &pid)
            .arg(&pid)
            .arg(&child_pipe_name) // ここで子プロセス用のパイプ名を渡す
            .arg("window")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        // 親プロセスは子プロセス用のパイプでサーバーを起動し、子からの接続を待つ
        let server = Server::start(&child_pipe_name)?;

        // メインプロセスは子プロセスにメッセージを送るためのクライアントを起動する
        let new_window_client = Client::start(&child_pipe_name)?;

        let new_window = Window {
            pipe_name: child_pipe_name,
            server: WindowServer::new(server, Some(child)),
            client: WindowClient::new(new_window_client),
        };

        self.windows.insert(window_type, new_window);

        Ok(())
    }
    pub fn send_window_command(
        &self,
        window_type: WindowType,
        command: InstanceMessage,
    ) -> Result<(), LxDosError> {
        if let Some(window) = self.windows.get(&window_type) {
            window.client.send(&command)?;
            return Ok(());
        }
        Err(LxDosError::Message(format!(
            "No client found for window of type {:?}",
            window_type
        )))
    }
}

impl Drop for WindowManager {
    fn drop(&mut self) {
        for window_struct in &self.windows {
            let (_, window) = window_struct;
            let mut child_process = window.server.child.lock().unwrap();
            let child_process = child_process.deref_mut();
            if let Some(child) = child_process {
                child.kill().unwrap();
            }
        }
    }
}
