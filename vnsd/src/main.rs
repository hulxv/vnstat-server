use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use app::Logger;
use log::{error, info, warn};

use tokio::{self, spawn, task::spawn_blocking};
use utils::unix_socket::{
    Message::{self, *},
    ServerMessage, UnixSocket,
};
use vnsd::{
    server::{Server, ServerHandlingError},
    Args,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    Logger::init();

    // let args = Args::parse();
    let mut listener = UnixSocket::bind("/tmp/vns.socket")
        .map_err(|e| {
            error!(
                "Cannot bind unix socket server: {}",
                e.root_cause().downcast_ref::<std::io::Error>().unwrap() // .message
            )
        })
        .unwrap();

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
        async {
            loop {
                listener.handle().await.map_err(|e| error!("{e}")).unwrap();
                match listener.receive().await {
                    Ok(message) => {
                        if let Ok(message) =
                            Message::from_str(message.as_str()).map_err(|err| error!("{err}"))
                        {
                            match message {
                                PauseServer => {
                                    warn!("Pause server...",);

                                    if let Err(err) = server.resume().await {
                                        error!("Cannot pause connections: {}", err.clone());
                                        loop {
                                            match listener
                                                .send(
                                                    ServerMessage::failed(
                                                        format!("{}", err.clone()).as_str(),
                                                    )
                                                    .as_str(),
                                                )
                                                .await
                                            {
                                                Err(ref e)
                                                    if e.root_cause()
                                                        .downcast_ref::<std::io::Error>()
                                                        .unwrap()
                                                        .kind()
                                                        == std::io::ErrorKind::WouldBlock =>
                                                {
                                                    continue;
                                                }
                                                Err(e) => {
                                                    error!("Cannot send message to client: {e}");
                                                }
                                                _ => (),
                                            };
                                            break;
                                        }
                                    } else {
                                        warn!(
                                            "Server accecping incoming connections has been pause"
                                        )
                                    }
                                }
                                ResumeServer => {
                                    info!("Resume server...",);
                                    if let Err(err) = server.resume().await {
                                        error!("Cannot resume connections: {}", err.clone());
                                        loop {
                                            match listener
                                                .send(
                                                    ServerMessage::failed(
                                                        format!("{}", err.clone()).as_str(),
                                                    )
                                                    .as_str(),
                                                )
                                                .await
                                            {
                                                Err(ref e)
                                                    if e.root_cause()
                                                        .downcast_ref::<std::io::Error>()
                                                        .unwrap()
                                                        .kind()
                                                        == std::io::ErrorKind::WouldBlock =>
                                                {
                                                    continue;
                                                }
                                                Err(e) => {
                                                    error!("Cannot send message to client: {e}");
                                                }
                                                _ => (),
                                            };
                                            break;
                                        }
                                    } else {
                                        info!(
                                            "Server accecping incoming connections has been resume"
                                        )
                                    }
                                }
                                StatusServer => {
                                    let (ip, port) = server.address();

                                    loop {
                                        match listener
                                            .send(
                                                ServerMessage::without_status(vec![
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
                                            Err(ref e)
                                                if e.root_cause()
                                                    .downcast_ref::<std::io::Error>()
                                                    .unwrap()
                                                    .kind()
                                                    == std::io::ErrorKind::WouldBlock =>
                                            {
                                                continue;
                                            }
                                            Err(e) => {
                                                error!("Cannot send server status: {e}");
                                                break;
                                            }
                                            _ => (),
                                        }
                                        break;
                                    }
                                }
                                ShutdownServer => {
                                    warn!("Shutdown server...");

                                    if let Err(err) = server.stop().await {
                                        error!("Cannot stop server: {}", err.clone());
                                        loop {
                                            match listener
                                                .send(
                                                    ServerMessage::failed(
                                                        format!("{}", err.clone()).as_str(),
                                                    )
                                                    .as_str(),
                                                )
                                                .await
                                            {
                                                Err(ref e)
                                                    if e.root_cause()
                                                        .downcast_ref::<std::io::Error>()
                                                        .unwrap()
                                                        .kind()
                                                        == std::io::ErrorKind::WouldBlock =>
                                                {
                                                    continue;
                                                }
                                                Err(e) => {
                                                    error!("Cannot send message to client: {e}");
                                                    break;
                                                }
                                                _ => (),
                                            };
                                            break;
                                        }
                                    } else {
                                        warn!("Server has been shutdown, you need to restart daemon to running it again.")
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    Err(ref e)
                        if e.root_cause()
                            .downcast_ref::<std::io::Error>()
                            .unwrap()
                            .kind()
                            == std::io::ErrorKind::WouldBlock =>
                    {
                        continue;
                    }
                    Err(e) => return Err(e),
                };
            }
        }
    );
    Ok(())
}
