use app::{Configs, Logger};
use clap::Parser;
use log::{error, info, warn};
use serde_json;
use tokio::{self, spawn};
use utils::unix_socket::{Request, Response, UnixSocket};
use vnsd::{
    cli::Args,
    server::{Server, ServerAddr},
    uds_request_handler::RequestHandler,
};


#[tokio::main]
async fn main() -> std::process::ExitCode {
    Logger::init();
    let args = Args::parse();
    match Configs::get_file_path() {
        Ok(path) => info!("configuration file located in: \"{path}\"",),
        Err(err) => {
            error!("Cannot locate configuration file: {err}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let configs = Configs::init().unwrap();

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
    let server = Server::new(ServerAddr::new(
        &args.ip.unwrap_or(configs.server().ip()),
        args.port.unwrap_or(configs.server().port()),
    ))
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
                    Ok(req) => {
                        if let Ok(req) = serde_json::from_str::<Request>(&req) {
                            let mut res = Response::new();

                            RequestHandler::new(&server, req, &mut res).handle().await;

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
    std::process::ExitCode::SUCCESS
}
