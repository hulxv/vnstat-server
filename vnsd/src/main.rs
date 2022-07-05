use app::Logger;
use log::{error, info, warn};
use serde_json;
use std::collections::HashMap;

use tokio::{self, spawn};
use utils::unix_socket::{Commands::*, Request, Response, ServerResponseMessage, UnixSocket};
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
            // TODO: refactoring handling UNIX connections
            loop {
                match listener.receive().await {
                    Ok(message) => {
                        if let Ok(message) = serde_json::from_str::<Request<String>>(&message) {
                            let mut res = Response::new();

                            match message.command {
                                PauseServer => {
                                    warn!("Pause server...",);

                                    if let Err(err) = server.pause().await {
                                        error!("Cannot pause connections: {}", err.clone());

                                        res.push(ServerResponseMessage::failed(&format!(
                                            "{}",
                                            err.clone()
                                        )));
                                    } else {
                                        let message =
                                            "Server accecping incoming connections has been pause.";
                                        warn!("{message}");

                                        res.push(ServerResponseMessage::success(message));
                                    }
                                }
                                ResumeServer => {
                                    info!("Resume server...",);
                                    if let Err(err) = server.resume().await {
                                        error!("Cannot resume connections: {}", err.clone());

                                        res.push(ServerResponseMessage::failed(&format!("{err}")));
                                    } else {
                                        let message =
                                                    "Server accecping incoming connections has been resume.";
                                        info!("{message}");

                                        res.push(ServerResponseMessage::failed(message));
                                    }
                                }
                                StatusServer => {
                                    let (ip, port) = server.address();
                                    let mut body = HashMap::new();

                                    body.insert("ip", ip);
                                    body.insert("port", port.to_string());
                                    body.insert("status", server.status().get_state().to_string());

                                    res.push(ServerResponseMessage::success(&format!("{body:?}")));
                                }
                                ShutdownServer => {
                                    warn!("Shutdown server...");

                                    if let Err(err) = server.stop().await {
                                        error!("Cannot stop server: {}", err.clone());
                                        res.push(ServerResponseMessage::failed(&format!("{err}")));
                                    } else {
                                        let message = "Server has been shutdown, you need to restart vnsd to running it again.";
                                        warn!("{message}");

                                        res.push(ServerResponseMessage::failed(message));
                                    }
                                }
                                BlockIPs => {
                                    info!("block {:?}", message.args);
                                    let db = InitDatabase::connect().unwrap();
                                    db.init().unwrap();
                                    for addr in message.args.iter() {
                                        match BlockList::block(db.conn(), addr) {
                                            Ok(_) => {
                                                info!("{addr} has been blocked");

                                                res.push(ServerResponseMessage::success(&format!(
                                                    "{addr} has been blocked"
                                                )));
                                            }
                                            Err(err) => {
                                                error!("Cannot block {addr}: {err}");

                                                res.push(ServerResponseMessage::failed(&format!(
                                                    "{err}"
                                                )));
                                            }
                                        }
                                    }
                                }
                                UnBlockIPs => {
                                    let db = InitDatabase::connect().unwrap();
                                    db.init().unwrap();
                                    for addr in message.args.iter() {
                                        match BlockList::unblock(db.conn(), addr) {
                                            Ok(_) => {
                                                info!("{addr} has been unblocked");
                                                res.push(ServerResponseMessage::success(&format!(
                                                    "{addr} has been unblocked"
                                                )));
                                            }
                                            Err(err) => {
                                                error!("Cannot unblock {addr}: {err}");

                                                res.push(ServerResponseMessage::failed(&format!(
                                                    "{err}"
                                                )));
                                            }
                                        }
                                    }
                                }
                                _ => (),
                            }
                            if let Err(e) =
                                listener.send(&format!("{}", serde_json::json!(res))).await
                            {
                                error!("Could send to unix stream: {e}");
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
