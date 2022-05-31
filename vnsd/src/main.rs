use std::{
    str::FromStr,
    thread::{self, spawn},
};

use app::Logger;
use clap::Parser;
use log::{error, info};

use vnsd::{
    server::Server,
    unix_socket::{
        Message::{self, RestartServer, RunServer, ShutdownServer},
        UnixServer,
    },
    Args,
};
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    Logger::init();
    let args = Args::parse();
    let listener = UnixServer::bind("/tmp/vns.socket")
        .map_err(|err| error!("{err}"))
        .unwrap();

    let _ = tokio::join!(
        async {
            tokio::signal::ctrl_c()
                .await
                .map_err(|e| error!("{e}"))
                .is_ok()
                .then(|| std::process::exit(0));
        },
        async {
            loop {
                match listener.handle().await {
                    Ok((message, ..)) => {
                        if let Ok(message) =
                            Message::from_str(message.as_str()).map_err(|err| error!("{err}"))
                        {
                            match message {
                                RunServer => {
                                    info!("Running server...",);
                                    spawn(|| {
                                        Server::default().run().unwrap();
                                    });
                                }
                                ShutdownServer => {
                                    info!("stop server...");
                                }
                                RestartServer => {
                                    info!("restart server...");
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => return Err(e),
                };
            }
            Ok(())
        }
    );
    Ok(())
}
