use serde_derive::{Deserialize, Serialize};
use std::{str::FromStr, string::ToString};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub command: Commands,
    pub args: Vec<String>,
}

impl Request {
    pub fn new(command: Commands, args: Vec<String>) -> Self {
        Self { command, args }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub messages: Vec<ServerResponseMessage>,
}

impl Response {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    pub fn push(&mut self, message: ServerResponseMessage) -> &mut Self {
        self.messages.push(message);
        self
    }
    pub fn pop(&mut self) -> &mut Self {
        self.messages.pop();
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Commands {
    ShutdownServer,
    RunServer,
    RestartServer,
    StatusServer,
    ResumeServer,
    PauseServer,
    BlockIPs,
    UnBlockIPs,
}

impl FromStr for Commands {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shutdown" | "server-shutdown" => Ok(Self::ShutdownServer),
            "restart" | "server-restart" => Ok(Self::RestartServer),
            "run" | "server-run" => Ok(Self::RunServer),
            "status" | "server-status" => Ok(Self::StatusServer),
            "pause" | "server-pause" => Ok(Self::PauseServer),
            "resume" | "server-resume" => Ok(Self::ResumeServer),
            "block" | "server-block" => Ok(Self::BlockIPs),
            "unblock" | "server-unblock" => Ok(Self::UnBlockIPs),
            _ => Err("invalid message"),
        }
    }
}

impl ToString for Commands {
    fn to_string(&self) -> String {
        match self {
            Self::ShutdownServer => "server-shutdown",
            Self::RestartServer => "serve-restart",
            Self::RunServer => "server-run",
            Self::PauseServer => "server-pause",
            Self::ResumeServer => "server-resume",
            Self::StatusServer => "server-status",
            Self::BlockIPs => "server-block",
            Self::UnBlockIPs => "server-unblock",
        }
        .to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerResponseStatus {
    Success,
    Failed,
}
impl ToString for ServerResponseStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Success => "Success",
            Self::Failed => "Failed",
        }
        .to_owned()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerResponseMessage {
    pub status: ServerResponseStatus,
    pub body: String,
}

impl ServerResponseMessage {
    pub fn success(body: &str) -> Self {
        Self {
            status: ServerResponseStatus::Success,
            body: body.to_owned(),
        }
    }

    pub fn failed(body: &str) -> Self {
        Self {
            status: ServerResponseStatus::Failed,
            body: body.to_owned(),
        }
    }

    pub fn new(status: ServerResponseStatus, body: &str) -> Self {
        Self {
            status,
            body: body.to_owned(),
        }
    }
}
impl ToString for ServerResponseMessage {
    fn to_string(&self) -> String {
        format!("{self:?}")
    }
}
