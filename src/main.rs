use app::log::Logger;
use clap::Parser;
use log::{error, warn};
use std::{str::FromStr, string::ToString};
use utils::unix_socket::{Message, UnixSocket};
use vns::cli::{Args, Commands, ServerCommands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    Logger::init();

    let mut socket = UnixSocket::connect("/tmp/vns.socket")
        .await
        .map_err(|e| error!("{e}"))
        .expect("");

    match args.commands {
        Some(Commands::Server { command }) => loop {
            match socket
                .send(
                    Message::from_str(format!("{command}").as_str())
                        .unwrap()
                        .to_string()
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
                Err(e) => error!("{e}"),
                Ok(_) => match command {
                    ServerCommands::Shutdown => {
                        warn!("Shutdown server gracefully, you will need to restart vns daemon to re-running http server");
                    }
                    ServerCommands::Status => loop {
                        match socket.receive().await {
                            Ok(message) => {
                                println!("{message:#?}",);
                                break;
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
                            Err(e) => error!("{e}"),
                        }
                    },
                    _ => (),
                },
            };
            break;
        },
        _ => {
            warn!("use --help flag to show available commands")
        }
    }

    Ok(())
}

// async fn send_message(stream: &UnixStream, message: &str) -> Result<(), std::io::Error> {
//     loop {
//         stream.writable().await?;
//         match stream.try_write(message.as_bytes()) {
//             Ok(n) => {
//                 break;
//             }
//             Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
//                 continue;
//             }
//             Err(e) => return Err(e),
//         };
//     }
//     Ok(())
// }
