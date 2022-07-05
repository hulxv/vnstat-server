use crate::server::{
    api::auth::database::{BlockList, InitDatabase},
    Server,
};
use log::*;
use std::collections::HashMap;
use utils::unix_socket::{Commands::*, Request, Response, ServerResponseMessage};

/// Handling request that coming from UNIX socket
pub struct RequestHandler<'a> {
    server: &'a Server,
    req: Request,
    res: &'a mut Response,
}

impl<'a> RequestHandler<'a> {
    pub fn new(server: &'a Server, req: Request, res: &'a mut Response) -> Self {
        Self { server, req, res }
    }
    pub async fn handle(&mut self) {
        match self.req.command {
            PauseServer => {
                self.on_pause_server().await;
            }
            ResumeServer => {
                self.on_resume_server().await;
            }
            ShutdownServer => {
                self.on_shutdown_server().await;
            }
            StatusServer => {
                self.on_status_server();
            }
            BlockIPs => {
                self.on_block_ip_addresses();
            }
            UnBlockIPs => {
                self.on_unblock_ip_addresses();
            }
            _ => (),
        }
    }

    async fn on_pause_server(&mut self) {
        warn!("Pause server...",);

        if let Err(err) = self.server.pause().await {
            error!("Cannot pause connections: {}", err.clone());

            self.res
                .push(ServerResponseMessage::failed(&format!("{}", err.clone())));
        } else {
            let message = "Server accecping incoming connections has been pause.";
            warn!("{message}");

            self.res.push(ServerResponseMessage::success(message));
        }
    }
    async fn on_resume_server(&mut self) {
        info!("Resume server...",);
        if let Err(err) = self.server.resume().await {
            error!("Cannot resume connections: {}", err.clone());

            self.res
                .push(ServerResponseMessage::failed(&format!("{err}")));
        } else {
            let message = "Server accecping incoming connections has been resume.";
            info!("{message}");

            self.res.push(ServerResponseMessage::success(message));
        }
    }
    fn on_status_server(&mut self) {
        let (ip, port) = self.server.address();
        let mut body = HashMap::new();

        body.insert("ip", ip);
        body.insert("port", port.to_string());
        body.insert("status", self.server.status().get_state().to_string());

        self.res
            .push(ServerResponseMessage::success(&format!("{body:?}")));
    }

    async fn on_shutdown_server(&mut self) {
        warn!("Shutdown server...");

        if let Err(err) = self.server.stop().await {
            error!("Cannot stop server: {}", err.clone());
            self.res
                .push(ServerResponseMessage::failed(&format!("{err}")));
        } else {
            let message = "Server has been shutdown.";
            warn!("{message}");

            self.res.push(ServerResponseMessage::success(message));
        }
    }

    fn on_block_ip_addresses(&mut self) {
        info!("block {:?}", self.req.args);
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();
        for addr in self.req.args.iter() {
            match BlockList::block(db.conn(), addr) {
                Ok(_) => {
                    info!("{addr} has been blocked");

                    self.res.push(ServerResponseMessage::success(&format!(
                        "{addr} has been blocked"
                    )));
                }
                Err(err) => {
                    error!("Cannot block {addr}: {err}");

                    self.res
                        .push(ServerResponseMessage::failed(&format!("{err}")));
                }
            }
        }
    }

    fn on_unblock_ip_addresses(&mut self) {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();
        for addr in self.req.args.iter() {
            match BlockList::unblock(db.conn(), addr) {
                Ok(_) => {
                    info!("{addr} has been unblocked");
                    self.res.push(ServerResponseMessage::success(&format!(
                        "{addr} has been unblocked"
                    )));
                }
                Err(err) => {
                    error!("Cannot unblock {addr}: {err}");

                    self.res.push(ServerResponseMessage::failed(&format!(
                        "Cannot unblock {addr}: {err}"
                    )));
                }
            }
        }
    }
}
