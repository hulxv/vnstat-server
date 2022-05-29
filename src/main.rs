use app::log::Logger;
use clap::Parser;
use log::warn;
use vns::cli::{Args, Commands, ServerCommands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    Logger::init();

    let _ = match args.commands {
        Commands::Server { command } => match command {
            ServerCommands::Run => {
                todo!()
            }
            ServerCommands::Shutdown => {
                todo!()
            }
            _ => (),
        },
        _ => {
            warn!("use --help flag to show available commands")
        }
    };

    Ok(())
}
