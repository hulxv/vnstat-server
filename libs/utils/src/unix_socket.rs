use anyhow::{anyhow, Result};
use log::warn;
use std::{
    collections::HashMap,
    fs::remove_file,
    path::Path,
    str::{from_utf8, FromStr},
    string::ToString,
};
use tokio::net::{UnixListener, UnixStream};

pub enum Message {
    ShutdownServer,
    RunServer,
    RestartServer,
    StatusServer,
    ResumeServer,
    PauseServer,
}
impl FromStr for Message {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shutdown" | "server-shutdown" => Ok(Self::ShutdownServer),
            "restart" | "server-restart" => Ok(Self::RestartServer),
            "run" | "server-run" => Ok(Self::RunServer),
            "status" | "server-status" => Ok(Self::StatusServer),
            "pause" | "server-pause" => Ok(Self::PauseServer),
            "resume" | "server-resume" => Ok(Self::ResumeServer),
            _ => Err("invalid message"),
        }
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        match self {
            Self::ShutdownServer => "server-shutdown",
            Self::RestartServer => "serve-restart",
            Self::RunServer => "server-run",
            Self::PauseServer => "server-pause",
            Self::ResumeServer => "server-resume",
            Self::StatusServer => "server-status",
        }
        .to_owned()
        // )
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum UnixSocketSide {
    Client,
    Server,
}

#[derive(Debug)]
pub struct UnixSocket {
    listener: Option<UnixListener>,
    stream: Option<UnixStream>,
    side: UnixSocketSide,
}

impl UnixSocket {
    /// bind unix socket
    pub fn bind(path: &str) -> Result<Self> {
        if Path::new(path).exists() {
            warn!("Unix listener address is exist, it will be removed and previous connection will broken.");
            match remove_file(path) {
                Err(e) => return Err(anyhow!(e)),
                Ok(_) => warn!("Unix listener address has been removed"),
            };
        }
        match UnixListener::bind(path) {
            Err(e) => Err(anyhow!(e)),
            Ok(listener) => Ok(Self {
                listener: Some(listener),
                stream: None,
                side: UnixSocketSide::Server,
            }),
        }
    }

    /// Connect to unix socket
    pub async fn connect(path: &str) -> Result<Self> {
        Ok(Self {
            listener: None,
            stream: Some(UnixStream::connect(path).await?),
            side: UnixSocketSide::Client,
        })
    }

    /// Handling socket connections before receive messages
    pub async fn accept(&mut self) -> Result<&mut Self> {
        match self.listener.as_ref().unwrap().accept().await {
            Ok((stream, _)) => {
                self.stream = Some(stream);
                Ok(self)
            }
            Err(err) => Err(anyhow!(err)),
        }
    }

    /// Recieve messages from stream
    pub async fn receive(&mut self) -> Result<String> {
        loop {
            if self.side.eq(&UnixSocketSide::Server) {
                if let Err(e) = self.accept().await {
                    return Err(anyhow!(e));
                }
            }

            self.stream.as_ref().unwrap().readable().await.unwrap();
            let mut buf = vec![0; 1024];
            match self.stream.as_ref().unwrap().try_read(&mut buf) {
                Ok(n) => {
                    buf.truncate(n);
                    return Ok(from_utf8(&buf).unwrap().to_owned());
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(anyhow!(e));
                }
            }
        }
    }

    /// Send messages to stream
    pub async fn send(&self, message: &str) -> Result<()> {
        loop {
            self.stream.as_ref().unwrap().writable().await?;
            if let Err(err) = self.stream.as_ref().unwrap().try_write(message.as_bytes()) {
                match err {
                    ref e if e.kind() == std::io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    e => {
                        return Err(anyhow!(e));
                    }
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}

pub struct ServerMessage;

impl ServerMessage {
    pub fn success(details: &str) -> String {
        format!(
            "{:?}",
            HashMap::from([("status", "success"), ("details", details)])
        )
    }
    pub fn failed(details: &str) -> String {
        format!(
            "{:?}",
            HashMap::from([("status", "failed"), ("details", details)])
        )
    }

    pub fn new(data: Vec<(&str, &str)>) -> String {
        let mut hash = HashMap::new();
        data.iter().for_each(|(k, v)| {
            hash.insert(k, v);
        });

        format!("{:?}", hash)
    }
}
