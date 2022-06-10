use std::{collections::HashMap, str::FromStr, time::Duration};

use clap::Parser;
use colored::Colorize;
use log::{error, warn};
use tokio::{select, time};

use app::log::Logger;
use utils::unix_socket::{Message, UnixSocket};
use vns::cli::{
    Args, Commands,
    ServerCommands::{Pause, Resume, Shutdown, Status},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    Logger::init();

    match args.commands {
        Some(Commands::Server { command }) => {
            let socket = UnixSocket::connect("/tmp/vns.socket")
                .await
                .map_err(|e| error!("{e}"))
                .expect("");

            loop {
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
                        Resume | Pause | Shutdown => {
                            if command == Shutdown {
                                warn!("Shutdown server gracefully, you will need to restart vns daemon to re-running http server");
                            }
                            select!(
                                _ = async {
                                    loop {

                                        match socket.receive().await {
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
                                            Ok(message) => {
                                                let hash: HashMap<String, String> = serde_json::from_str(message.as_str()).unwrap();
                                                println!("{}: {}",
                                                    match hash.get("status").unwrap().to_lowercase().as_str(){
                                                        "failed" => "Failed".red().bold(),
                                                        "success" => "Success".green().bold(),
                                                        e => e.bold()
                                                    },
                                                    hash.get("details").unwrap()
                                                );
                                            }
                                        }
                                        break;

                                    }
                                } => {}
                                _ = async {
                                    time::sleep(Duration::from_millis(4000)).await;
                                } => {}
                            );
                        }
                        Status => select!(_ = async{loop {
                                match socket.receive().await {
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
                                    Ok(message) => {
                                        let hash: HashMap<String, String> =
                                            serde_json::from_str(message.as_str()).unwrap();

                                        let (ip, port) =
                                            (hash.get("ip").unwrap(), hash.get("port").unwrap());

                                        match hash.get("status").unwrap().to_lowercase().as_str() {
                                            "active" => {
                                                println!(
                                                    "{} ({}) {}:",
                                                    "Active".green().bold(),
                                                    "Running".yellow().bold(),
                                                    "on".bold(),
                                                );
                                                println!(
                                                    "{: >1}  {}",
                                                    "",
                                                    format!("{:<5} {}", "IP", ip.yellow(),).bold()
                                                );
                                                println!(
                                                    "{: >1}  {}",
                                                    "",
                                                    format!("{:<5} {}", "PORT", port.yellow()).bold()
                                                );
                                            }
                                            "idle" => {
                                                println!(
                                                    "{} ({})",
                                                    "Idle".cyan().bold(),
                                                    "Paused".blue().bold(),
                                                );

                                                println!(
                                                    "\n{} ",
                                                    format!(
                                                        "{}\n {} \n{}:  \n $ {}",
                                                        "Note".bright_blue(),
                                                        "To resume incoming connections",
                                                        "run".yellow(),
                                                        "vns server resume",
                                                    )
                                                    .bold()
                                                )
                                            }
                                            "inactive" => {
                                                println!(
                                                    "{} ({})",
                                                    "Inactive".yellow().bold(),
                                                    "STOPPED".red().bold(),
                                                );
                                                println!(
                                                    "\n{}\n {}",
                                                    "Note".bright_blue().bold(),
                                                    format!(
                                                        "Restart {} to re-running the server.",
                                                        "vns daemon".yellow(),
                                                    )
                                                )
                                            }
                                            _ => (),
                                        };
                                    }
                                }
                                break;
                            }} => {},
                        _ = async{
                                time::sleep(Duration::from_millis(4000)).await;
                            } => error!("Cannot recieve server status: connection timeout")
                        ),
                    },
                };
                break;
            }
        }
        None => {
            warn!("use --help flag to show available commands")
        }
    }

    Ok(())
}
