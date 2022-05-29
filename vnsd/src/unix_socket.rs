use anyhow::Result;
use log::error;
use std::{
    fs::remove_file,
    path::Path,
    str::{from_utf8, FromStr},
};
use tokio::net::{unix::SocketAddr, UnixListener, UnixStream};

pub enum Message {
    ShutdownServer,
    RunServer,
    RestartServer,
}
impl FromStr for Message {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shutdown" | "shutdown-server" => Ok(Self::ShutdownServer),
            "restart" | "restart-server" => Ok(Self::RestartServer),
            "run" | "run-server" => Ok(Self::RunServer),
            _ => Err("invalid message"),
        }
    }
}
pub struct UnixServer;

impl UnixServer {
    pub fn bind(path: &str) -> Result<UnixListener> {
        if Path::new(path).exists() {
            remove_file(path).unwrap();
        }
        Ok(UnixListener::bind(path).unwrap())
    }
    pub async fn handle(
        listener: &UnixListener,
    ) -> Result<(String, UnixStream, SocketAddr), std::io::Error> {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                // Wait for the socket to be readable
                stream.readable().await.unwrap();
                let mut buf = vec![0; 1024];

                match stream.try_read(&mut buf) {
                    Ok(n) => {
                        buf.truncate(n);
                        Ok((from_utf8(&buf).unwrap().to_owned(), stream, _addr))
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }
}
