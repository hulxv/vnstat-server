mod communication;

pub use communication::*;

use anyhow::{anyhow, Result};
use log::warn;
use std::{
    fs::{remove_file, set_permissions, Permissions},
    os::unix::fs::PermissionsExt,
    path::Path,
    str::from_utf8,
};
use tokio::net::{UnixListener, UnixStream};

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

        let sock = Self {
            listener: Some(UnixListener::bind(path)?),
            stream: None,
            side: UnixSocketSide::Server,
        };
        set_permissions(path, Permissions::from_mode(0o666))?;
        Ok(sock)
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
    pub async fn streaming(&mut self) -> Result<&mut Self> {
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
                self.streaming().await?;
            }

            self.stream.as_ref().unwrap().readable().await?;
            let mut buf = vec![0; 1024];
            match self.stream.as_ref().unwrap().try_read(&mut buf) {
                Ok(n) => {
                    buf.truncate(n);
                    return Ok(from_utf8(&buf)?.to_owned());
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
