use app::log::Logger;
use clap::Parser;
use log::{error, warn};
use tokio::net::UnixStream;
use vns::cli::{Args, Commands, ServerCommands};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    Logger::init();

    let stream = UnixStream::connect("/tmp/vns.socket").await?;

    match args.commands {
        Some(Commands::Server { command }) => {
            send_message(&stream, format!("{command}").as_str())
                .await
                .map_err(|e| error!("{e}"))
                .unwrap();
        }
        _ => {
            warn!("use --help flag to show available commands")
        }
    };

    Ok(())
}

async fn send_message(stream: &UnixStream, message: &str) -> Result<(), std::io::Error> {
    loop {
        stream.writable().await?;
        match stream.try_write(message.as_bytes()) {
            Ok(n) => {
                break;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => return Err(e),
        };
    }
    Ok(())
}
