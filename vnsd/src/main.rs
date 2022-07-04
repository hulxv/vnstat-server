use app::Logger;
use log::{error, info, warn};
use serde_json;

use tokio::{self, spawn};
use utils::unix_socket::{DaemonCommands::*, DaemonRequest, ServerMessage, UnixSocket};
use vnsd::server::{
    api::auth::database::{BlockList, Create, InitDatabase},
    Server,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    Logger::init();

    let sock_path = "/tmp/vnsd.sock";
    let mut listener = match UnixSocket::bind(sock_path) {
        Err(e) => {
            error!("Cannot bind unix server: {e}");
            std::process::exit(1);
        }
        Ok(lis) => {
            info!("uds listening on '{sock_path}'");
            lis
        }
    };
    let server = Server::new()
        .map_err(|e| error!("Cannot bind http server: {e}"))
        .unwrap();

    spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .map_err(|e| error!("{e}"))
            .is_ok()
            .then(|| std::process::exit(0));
    });

    let _: (_, Result<(), anyhow::Error>) = tokio::join!(
        // Running HTTP server
        async {
            let (ip, port) = server.address();

            info!("Server running on http://{ip}:{port}");
            server
                .run()
                .await
                .map_err(|e| error!("Cannot run the server: {e}"))
                .is_err()
                .then(|| warn!("Server has been disconnected"));
        },
        // Listening to UNIX socket commands
        async {
            loop {
                match listener.receive().await {
                    Ok(message) => {
                        if let Ok(message) = serde_json::from_str::<DaemonRequest<String>>(&message)
                        {
                            match message.command {
                                PauseServer => {
                                    warn!("Pause server...",);

                                    if let Err(err) = server.pause().await {
                                        error!("Cannot pause connections: {}", err.clone());

                                        if let Err(e) = listener
                                            .send(
                                                ServerMessage::failed(
                                                    format!("{}", err.clone()).as_str(),
                                                )
                                                .as_str(),
                                            )
                                            .await
                                        {
                                            error!("Couldn't send to unix stream: {e}");
                                        }
                                    } else {
                                        let message =
                                            "Server accecping incoming connections has been pause.";
                                        warn!("{message}");
                                        if let Err(e) = listener
                                            .send(ServerMessage::success(message).as_str())
                                            .await
                                        {
                                            error!("Couldn't send to unix stream: {e}");
                                        }
                                    }
                                }
                                ResumeServer => {
                                    info!("Resume server...",);
                                    if let Err(err) = server.resume().await {
                                        error!("Cannot resume connections: {}", err.clone());
                                        if let Err(e) = listener
                                            .send(
                                                ServerMessage::failed(
                                                    format!("{}", err.clone()).as_str(),
                                                )
                                                .as_str(),
                                            )
                                            .await
                                        {
                                            error!("Could send to unix stream: {e}");
                                        }
                                    } else {
                                        let message =
                                            "Server accecping incoming connections has been resume.";
                                        info!("{message}");

                                        if let Err(e) = listener
                                            .send(ServerMessage::success(message).as_str())
                                            .await
                                        {
                                            error!("Couldn't send to unix stream: {e}");
                                        }
                                    }
                                }
                                StatusServer => {
                                    let (ip, port) = server.address();

                                    if let Err(e) = listener
                                        .send(
                                            ServerMessage::new(vec![
                                                (
                                                    "status",
                                                    server
                                                        .status()
                                                        .get_state()
                                                        .to_string()
                                                        .as_str(),
                                                ),
                                                ("ip", ip.as_str()),
                                                ("port", port.to_string().as_str()),
                                            ])
                                            .as_str(),
                                        )
                                        .await
                                    {
                                        error!("Could send to unix stream: {e}");
                                    }
                                }
                                ShutdownServer => {
                                    warn!("Shutdown server...");

                                    if let Err(err) = server.stop().await {
                                        error!("Cannot stop server: {}", err.clone());
                                        if let Err(e) = listener
                                            .send(
                                                ServerMessage::failed(
                                                    format!("{}", err.clone()).as_str(),
                                                )
                                                .as_str(),
                                            )
                                            .await
                                        {
                                            error!("Could send to unix stream: {e}");
                                        }
                                    } else {
                                        let message = "Server has been shutdown, you need to restart vnsd to running it again.";
                                        warn!("{message}");
                                        if let Err(e) = listener
                                            .send(ServerMessage::success(message).as_str())
                                            .await
                                        {
                                            error!("Couldn't send to unix stream: {e}");
                                        }
                                    }
                                }
                                BlockIPs => {
                                    info!("block {:?}", message.args);
                                    let db = InitDatabase::connect().unwrap();
                                    db.init().unwrap();
                                    for addr in message.args.iter() {
                                        BlockList::new(db.conn(), addr)
                                            .create(db.conn())
                                            .is_ok()
                                            .then(|| info!("{addr} has been blocked"));
                                    }

                                    if let Err(e) = listener
                                        .send(
                                            ServerMessage::success("Blocking was success").as_str(),
                                        )
                                        .await
                                    {
                                        error!("Couldn't send to unix stream: {e}");
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    Err(e) => {
                        error!("{e}");
                    }
                };
            }
        }
    );
    Ok(())
}
