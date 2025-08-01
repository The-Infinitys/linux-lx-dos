use crate::LxDosError;
use instance_pipe::{Client, Event, Server};
use std::collections::HashMap;
use std::env;
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
    pub fn new(server: Server, child: Child) -> Self {
        Self {
            server: Arc::new(Mutex::new(server)),
            child: Arc::new(Mutex::new(Some(child))),
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn poll_event(&self) -> Result<Vec<InstanceMessage>, LxDosError> {
        let mut server = self
            .server
            .lock()
            .map_err(|e| LxDosError::Message(e.to_string()))?;
        let mut messages = Vec::new();

        // Check for new connections
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

        // Poll messages from existing clients
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

pub struct WindowManager {
    pipe_name: String,
    // Use HashMap to easily identify servers and clients by pipe name
    servers: HashMap<String, WindowServer>,
    clients: HashMap<String, WindowClient>,
    open_windows: HashMap<WindowType, String>, // Tracks window type to pipe_name
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
            servers: HashMap::new(),
            clients: HashMap::new(),
            open_windows: HashMap::new(),
        }
    }

    pub fn start_server(&mut self) -> Result<(), LxDosError> {
        let server = Server::start(&self.pipe_name)?;
        let child = Command::new("true").spawn()?;
        self.servers
            .insert(self.pipe_name.clone(), WindowServer::new(server, child));
        Ok(())
    }

    pub fn poll_event(&mut self) -> Result<Vec<InstanceMessage>, LxDosError> {
        let mut messages = Vec::new();
        let mut pipes_to_close = Vec::new();

        for (__pipe_name, server) in &self.servers {
            for message in server.poll_event()? {
                if let InstanceMessage::CloseWindow {
                    pipe_name: ref closed_pipe_name,
                } = message
                {
                    pipes_to_close.push(closed_pipe_name.clone());
                }
                messages.push(message);
            }
        }

        // After polling all servers, clean up based on the received CloseWindow messages.
        for closed_pipe_name in pipes_to_close {
            println!(
                "Cleaning up resources for closed window: {}",
                closed_pipe_name
            );
            self.servers.remove(&closed_pipe_name);
            self.clients.remove(&closed_pipe_name);
            self.open_windows.retain(|_, v| v != &closed_pipe_name);
        }

        Ok(messages)
    }

    pub fn open_window(&mut self, window_type: WindowType) -> Result<(), LxDosError> {
        if self.open_windows.contains_key(&window_type) {
            println!("Window of type {:?} is already open", window_type);
            return Ok(());
        }

        let current_exe = env::current_exe()?;
        let pid = std::process::id().to_string();
        let child = Command::new(current_exe)
            .env("LXDOS_BACKEND", &pid)
            .arg(&pid)
            .arg(&self.pipe_name)
            .arg("window")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        let child_pipe_name = format!("{}_{}", self.pipe_name, child.id());
        let server = Server::start(&child_pipe_name)?;
        self.servers
            .insert(child_pipe_name.clone(), WindowServer::new(server, child));

        std::thread::sleep(std::time::Duration::from_millis(100));

        let client = Client::start(&self.pipe_name)?;
        client.send(&InstanceMessage::OpenWindow {
            pipe_name: child_pipe_name.clone(),
            window_type: window_type.clone(),
        })?;
        self.clients
            .insert(child_pipe_name.clone(), WindowClient::new(client));

        self.open_windows.insert(window_type, child_pipe_name);

        Ok(())
    }

    pub fn send_window_command(
        &mut self,
        window_type: WindowType,
        command: InstanceMessage,
    ) -> Result<(), LxDosError> {
        if let Some(pipe_name) = self.open_windows.get(&window_type) {
            if let Some(client) = self.clients.get(pipe_name) {
                client.send(&command)?;
                return Ok(());
            }
        }
        Err(LxDosError::Message(format!(
            "No client found for window of type {:?}",
            window_type
        )))
    }
}
