use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use app::Logger;
use log::{error, info, warn};

use tokio::{self, spawn, task::spawn_blocking};
use utils::unix_socket::{
    Message::{self, PauseServer, ResumeServer, ShutdownServer, StatusServer},
    ServerMessage, UnixSocket,
};
use vnsd::{
    server::{Server, ServerStatus, ServerStatusState},
    Args,
};

#[tokio::main]

async fn main() -> Result<(), std::io::Error> {
    Logger::init();

    // let args = Args::parse();
    let listener = Arc::new(Mutex::new(
        UnixSocket::bind("/tmp/vns.socket")
            .map_err(|err| error!("Cannot bind unix socket server: {err}"))
            .unwrap(),
    ));

    let server = Server::default();
    let server_runner = server
        .run()
        .map_err(|e| error!("Cannot bind http server: {e}"))
        .unwrap();
    let server_status = ServerStatus::new(ServerStatusState::InActive);
    let server_handler = Arc::new(Mutex::new(server_runner.handle()));

    spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .map_err(|e| error!("{e}"))
            .is_ok()
            .then(|| std::process::exit(0));
    });

    let _: ((), Result<(), anyhow::Error>) = tokio::join!(
        async {
            // let server = Arc::clone(&server);
            let (ip, port) = server.address();
            server_status.active();

            info!("Server running on http://{ip}:{port}");
            server_runner
                .await
                .map_err(|e| error!("{e}"))
                .is_err()
                .then(|| {
                    server_status.inactive();
                    warn!("Server has been disconnected");
                });
        },
        async {
            loop {
                let lisn = Arc::new(&listener);
                let mut lisn = lisn.lock().unwrap();
                (*lisn).handle().await.map_err(|e| error!("{e}")).unwrap();
                match lisn.receive().await {
                    Ok(message) => {
                        if let Ok(message) =
                            Message::from_str(message.as_str()).map_err(|err| error!("{err}"))
                        {
                            match message {
                                PauseServer => {
                                    warn!("Pause server...",);
                                    let handler = Arc::clone(&server_handler);
                                    spawn_blocking(move || {
                                        let svr = handler.lock().unwrap();
                                        svr.pause()
                                    })
                                    .await
                                    .map_err(|e| error!("{e}"))
                                    .is_ok()
                                    .then(|| {
                                        server_status.idle();
                                        warn!(
                                            "Server accecping incoming connections has been pause"
                                        )
                                    });
                                }
                                ResumeServer => {
                                    info!("Resume server...",);
                                    let handler = Arc::clone(&server_handler);
                                    spawn_blocking(move || {
                                        let svr = handler.lock().unwrap();
                                        svr.resume()
                                    })
                                    .await
                                    .map_err(|e| error!("{e}"))
                                    .is_ok()
                                    .then(|| {
                                        server_status.active();
                                        info!(
                                            "Server accecping incoming connections has been resume"
                                        )
                                    });
                                }
                                StatusServer => {
                                    let (ip, port) = server.address();
                                    loop {
                                        match lisn
                                            .send(
                                                ServerMessage::without_status(vec![
                                                    (
                                                        "status",
                                                        server_status
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
                                            Err(e) => error!("Cannot send server status: {e}"),
                                            _ => (),
                                        }
                                        break;
                                    }
                                    // });
                                }
                                ShutdownServer => {
                                    warn!("Shutdown server...");
                                    let handler = Arc::clone(&server_handler);
                                    spawn_blocking(move || {
                                        let svr = handler.lock().unwrap();
                                        svr.stop(true)
                                    })
                                    .await
                                    .map_err(|e| error!("{e}"))
                                    .is_ok().then(|| {
                                        server_status.inactive();
                                         warn!("Server has been shutdown, you need to restart daemon to running it again.");
                                    });
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
