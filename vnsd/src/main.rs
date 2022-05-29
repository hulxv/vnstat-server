use app::Logger;
use clap::Parser;
use log::{error, info};
use tokio;
use vnsd::server::Server;
use vnsd::Args;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    Logger::init();

    info!("Running server...",);
    tokio::task::spawn_blocking(|| match Server::default().run() {
        Err(e) => error!("{e}"),
        _ => (),
    })
    .await?;
    Ok(())
}
