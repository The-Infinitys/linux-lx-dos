use crate::LxDosError;
use instance_pipe::{Client, Server};
use std::env;
use std::process::{Child, Command, Stdio};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum InstanceMessage {
    OpenWindow { pipe_name: String },
}

pub struct WindowServer {
    server: Server,
    child: Option<Child>,
}

impl WindowServer {
    pub fn new(server: Server, child: Child) -> Self {
        Self {
            server,
            child: Some(child),
        }
    }
    pub fn recv(&mut self) -> InstanceMessage {
        self.server.accept().unwrap().recv().unwrap()
    }
}

impl Drop for WindowServer {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            if let Err(e) = child.kill() {
                log::error!("Failed to kill child process: {}", e);
            }
            if let Err(e) = child.wait() {
                log::error!("Failed to wait for child process: {}", e);
            }
        }
    }
}

pub struct WindowClient {
    #[allow(dead_code)]
    client: Client,
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
        let server = Server::new(&self.pipe_name)?;
        self.servers
            .push(WindowServer::new(server, Command::new("true").spawn()?)); // ダミー子プロセス
        Ok(())
    }
    pub fn poll_event(&mut self) -> Vec<InstanceMessage> {
        let mut result = Vec::with_capacity(self.servers.len());
        for server in &mut self.servers {
            result.push(server.recv());
        }
        result
    }
    pub fn open_window(&mut self) -> Result<(), LxDosError> {
        // バックエンドプロセスを起動
        let current_exe = env::current_exe()?;
        println!("{}", current_exe.display());
        let pid = std::process::id().to_string();
        let child = Command::new(current_exe)
            .env("LXDOS_BACKEND", &pid)
            .arg(&pid)
            .arg(&self.pipe_name)
            .arg("window")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        // 子プロセスごとの一意なパイプ名
        let child_pipe_name = format!("{}_{}", self.pipe_name, child.id());
        let server = Server::new(&child_pipe_name)?;
        self.servers.push(WindowServer::new(server, child));

        // パイプクライアントを作成してメッセージを送信
        let client = Client::connect(&self.pipe_name)?;
        client.send(&InstanceMessage::OpenWindow {
            pipe_name: child_pipe_name,
        })?;

        self.clients.push(WindowClient { client });
        Ok(())
    }
}
