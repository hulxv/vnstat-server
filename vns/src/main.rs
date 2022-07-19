use std::{str::FromStr, time::Duration};

use app::{log::Logger, UDS_ADDRESS};
use clap::Parser;
use colorful::Colorful;
use comfy_table::{presets::UTF8_FULL, Table};
use log::{error, warn};
use serde_derive::Deserialize;
use serde_json::json;
use tokio::{select, time};
use utils::unix_socket::{
    Commands as UnixSocketCommands, Request, Response, ServerResponseStatus, UnixSocket,
};
use vns::cli::{
    Args, Commands, List as ListType,
    ServerCommands::{self, *},
};

const TIME_OF_WAITING_RESPONSE_FROM_UNIX_SERVER: u64 = 6000; // By Milliseconds

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    Logger::init();

    match args.commands {
        Some(Commands::Server { command }) => {
            let mut socket = UnixSocket::connect(UDS_ADDRESS)
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
                Ok(_) => {
                    if command == Shutdown {
                        warn!("Shutdown server gracefully, you will need to restart vns daemon to re-running http server");
                    }
                    select!(
                        _ = async {
                            match socket.receive().await {
                                Err(e) => error!("Cannot receive response from unix server: {e}"),
                                Ok(res) => {
                                    let res = serde_json::from_str::<Response>(&res).unwrap();
                                    handle_response(command, res);
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
            };
        }
        None => {
            println!(
                "hint: use {} flag to show available commands",
                "--help".yellow().bold()
            )
        }
    }

    Ok(())
}

fn handle_response(command: ServerCommands, res: Response) {
    match command {
        Status => {
            #[derive(Deserialize)]
            struct Status {
                status: String,
                ip: String,
                port: String,
            }
            let res: Status = serde_json::from_str(&res.messages[0].body).unwrap();

            println!(
                "{:<7} {}",
                "Status".white(),
                match res.status.to_lowercase().as_str() {
                    "active" => format!("{} ({})", "Active", "Running".green()),
                    "inactive" => format!("{} ({})", "InActive", "Stopped".red()),
                    "idle" => format!("{} ({})", "Idle", "Paused".blue()),
                    _ => "".to_owned(),
                }
            );
            println!("{:<7} {}", "IP".white(), res.ip);
            println!("{:<7} {}", "PORT".white(), res.port);
        }
        List { list } => {
            let mut table = Table::new();
            match list {
                ListType::Block => {
                    #[derive(Deserialize)]
                    struct Row {
                        pub ip_addr: String,
                        pub blocked_at: String,
                    }

                    let data: Vec<Row> = serde_json::from_str(&res.messages[0].body).unwrap();
                    table
                        .load_preset(UTF8_FULL)
                        .set_header(["IP address", "Blocked at"]);
                    for row in data {
                        table.add_row([row.ip_addr, row.blocked_at]);
                    }
                }
                ListType::Connections => {
                    #[derive(Deserialize)]

                    struct Row {
                        pub uuid: String,
                        pub ip_addr: String,
                        pub user_agent: String,
                        pub connected_at: String,
                    }

                    let data: Vec<Row> = serde_json::from_str(&res.messages[0].body).unwrap();
                    table.load_preset(UTF8_FULL).set_header([
                        "UUID",
                        "IP address",
                        "User Agent",
                        "Connected at",
                    ]);
                    for row in data {
                        table.add_row([row.uuid, row.ip_addr, row.user_agent, row.connected_at]);
                    }
                }
            }
            println!("{table}");
        }

        _ => {
            for message in res.messages.iter() {
                println!(
                    "[{}] {}",
                    match message.status {
                        ServerResponseStatus::Failed => "Failed".red(),
                        ServerResponseStatus::Success => "Success".green(),
                    },
                    message.body
                );
            }
        }
    };
}
