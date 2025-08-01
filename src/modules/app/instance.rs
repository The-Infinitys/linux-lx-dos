use crate::LxDosError;
use instance_pipe::{Client, Event, Server};
use std::env;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum InstanceMessage {
    OpenWindow { pipe_name: String },
    CloseWindow { pipe_name: String },
}

#[derive(Clone)]
pub struct WindowServer {
    server: Arc<Mutex<Server>>,
    child: Arc<Mutex<Option<Child>>>,
    clients: Arc<Mutex<Vec<Client>>>, // Track connected clients
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
        let mut server = self.server.lock().map_err(|e| LxDosError::Message(e.to_string()))?;
        let mut messages = Vec::new();

        // Check for new connections
        match server.poll_event() {
            Ok(Some(Event::ConnectionAccepted(client))) => {
                println!("New client connected to server");
                // Store the client
                let mut clients = self.clients.lock().map_err(|e| LxDosError::Message(e.to_string()))?;
                clients.push(client.clone());
                // Send an initial OpenWindow message to the new client
                client.send(&InstanceMessage::OpenWindow {
                    pipe_name: "".to_string(),
                })?;
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
        let mut clients = self.clients.lock().map_err(|e| LxDosError::Message(e.to_string()))?;
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

    pub fn accept_and_receive(&self) -> Result<Vec<InstanceMessage>, LxDosError> {
        // Deprecated in favor of poll_event
        self.poll_event()
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
        let client = self.client.lock().map_err(|e| LxDosError::Message(e.to_string()))?;
        client.send(message)?;
        Ok(())
    }

    pub fn poll_event(&self) -> Result<Vec<InstanceMessage>, LxDosError> {
        let mut client = self.client.lock().map_err(|e| LxDosError::Message(e.to_string()))?;
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
    servers: Vec<WindowServer>,
    clients: Vec<WindowClient>,
}

impl WindowManager {
    pub fn new() -> Self {
        let pipe_name = format!("lxdos_pipe_{}", std::process::id());
        Self {
            pipe_name,
            servers: Vec::new(),
            clients: Vec::new(),
        }
    }

    pub fn start_server(&mut self) -> Result<(), LxDosError> {
        let server = Server::start(&self.pipe_name)?;
        let child = Command::new("true").spawn()?; // ダミー子プロセス
        self.servers.push(WindowServer::new(server, child));
        Ok(())
    }

    pub fn poll_event(&mut self) -> Result<Vec<InstanceMessage>, LxDosError> {
        let mut messages = Vec::new();
        for server in &mut self.servers {
            messages.extend(server.poll_event()?);
        }
        Ok(messages)
    }

    pub fn open_window(&mut self) -> Result<(), LxDosError> {
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
        self.servers.push(WindowServer::new(server, child));

        // Give the child process time to start its client
        std::thread::sleep(std::time::Duration::from_millis(100));

        let client = Client::start(&self.pipe_name)?;
        client.send(&InstanceMessage::OpenWindow {
            pipe_name: child_pipe_name.clone(),
        })?;
        self.clients.push(WindowClient::new(client));

        Ok(())
    }
}
