use std::str::FromStr;

use app::Logger;
use clap::Parser;
use log::{error, info};
use tokio::net::UnixListener;

use vnsd::{
    server::Server,
    unix_socket::{
        Message::{self, RestartServer, RunServer, ShutdownServer},
        UnixServer,
    },
    Args,
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::init();
    let args = Args::parse();
    let listener = UnixServer::bind("/tmp/vns.socket")
        .map_err(|err| error!("{err}"))
        .unwrap();

    loop {
        let (message, ..) = UnixServer::handle(&listener).await.unwrap();
        if let Ok(message) = Message::from_str(message.as_str()).map_err(|err| error!("{err}")) {
            match message {
                RunServer => {
                    info!("running server...")
                }
                ShutdownServer => {
                    info!("stop server...")
                }
                RestartServer => {
                    info!("restart server...")
                }
            }
        };
    }

    // info!("Running server...",);
    // tokio::task::spawn_blocking(|| match Server::default().run() {
    //     Err(e) => error!("{e}"),
    //     _ => (),
    // })
    // .await?;
    Ok(())
}
