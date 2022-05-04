use clap::Parser;
use log::{error, info, warn};
use std::{
    io::{self, Write},
    process, thread,
};
use vcs::app::{
    args::{Args, Commands, ServerCommands},
    log::Logger,
};
use vcs::*;

pub fn cli_runner() {
    info!(
        "VCS is running, Use it by enter commands or type \".help\" to show available commands or type \"q\" to quit."
    );
    loop {
        print!("vcs ~> ",);

        io::stdout().flush().expect("couldn't flushing");
        let mut input = String::new();

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().to_lowercase().as_str() {
                "run" => {
                    thread::spawn(|| run_server().unwrap());
                }
                "exit" | "q" | "quit" => {
                    warn!("Program has been terminated");
                    process::exit(1)
                }
                i if !i.is_empty() => error!("command not found: {}", i),

                _ => (),
            },
            Err(err) => error!("\nError: {}", err),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    Logger::init();

    let _ = match args.commands {
        Commands::Server { command } => match command {
            ServerCommands::Run => {
                warn!("Running server...",);
                tokio::task::spawn_blocking(|| match run_server() {
                    Err(e) => error!("{e}"),
                    _ => (),
                })
                .await?;
            }
            // ServerCommands::Shutdown => Ok(()),
            _ => (),
        },
        _ => {
            warn!("use --help flag to show available commands")
        }
    };

    Ok(())
}
