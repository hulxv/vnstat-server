use std::{collections::HashMap, str::FromStr, time::Duration};

use app::log::Logger;
use clap::Parser;
use colored::Colorize;
use log::{error, warn};
use tokio::{select, time};
use utils::unix_socket::{
    Commands as UnixSocketCommands, Request, Response, ServerResponseStatus, UnixSocket,
};
use vns::cli::{Args, Commands, ServerCommands::*};

const TIME_OF_WAITING_RESPONSE_FROM_UNIX_SERVER: u64 = 6000; // By Milliseconds
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    Logger::init();

    match args.commands {
        Some(Commands::Server { command }) => {
            let mut socket = UnixSocket::connect("/tmp/vnsd.sock")
                .await
                .map_err(|e| error!("{e}"))
                .unwrap();
            let message = Request::new(
                UnixSocketCommands::from_str(&command.to_string()).unwrap(),
                match command.clone() {
                    Block { addresses } | UnBlock { addresses } => addresses,
                    _ => vec![],
                },
            );

            match socket.send(&format!("{}", json!(message))).await {
                Err(e) => error!("Couldn't send to unix stream: {e}"),
                Ok(_) => match command {
                    Resume | Pause | Shutdown | Block { .. } | UnBlock { .. } => {
                        if command == Shutdown {
                            warn!("Shutdown server gracefully, you will need to restart vns daemon to re-running http server");
                        }
                        select!(
                            _ = async {
                                match socket.receive().await {
                                    Err(e) => error!("Cannot recieve response from unix server: {e}"),
                                    Ok(res) => {
                                        let res: Response = serde_json::from_str(&res).unwrap();
                                        println!("{res:#?}");

                                        for message in res.messages.iter() {
                                                println!("[{}] {}",
                                                match message.status {
                                                    ServerResponseStatus::Failed => "Failed".red().bold(),
                                                    ServerResponseStatus::Success => "Success".green().bold(),
                                                },
                                                message.body.bold()
                                            );
                                        }
                                    }
                                }
                            } => {}
                            _ = async {
                                time::sleep(Duration::from_millis(TIME_OF_WAITING_RESPONSE_FROM_UNIX_SERVER)).await;
                            } => {
                                error!("No response from unix server: connection timeout.")
                            }
                        );
                    }
                    Status => select!(_ = async{
                            match socket.receive().await {
                                Err(e) => error!("Cannot recieve response from unix stream: {e}"),
                                Ok(res) => {
                                    let res = serde_json::from_str::<Response>(&res).unwrap();
                                    let hash:  HashMap<String,String> =
                                        serde_json::from_str(&res.messages[0].body).unwrap();

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
                        } => {},
                    _ = async{
                            time::sleep(Duration::from_millis(TIME_OF_WAITING_RESPONSE_FROM_UNIX_SERVER)).await;
                        } => error!("No response from unix server: connection timeout")
                    ),
                },
            };
        }
        None => {
            warn!("use --help flag to show available commands")
        }
    }

    Ok(())
}
