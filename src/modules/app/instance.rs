use instance_pipe::{Client, Server};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum InstanceMessage {}

pub struct WindowServer {
    server: Server,
}
pub struct WindowClient {
    client: Client,
}
pub struct WindowManager {}
impl WindowManager {
    pub fn new() -> Self {
        Self {}
    }
}
