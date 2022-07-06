use clap::{Parser, Subcommand};
use std::fmt::Display;
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// To control your vns server
    Server {
        #[clap(subcommand)]
        command: ServerCommands,
    },
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Subcommand)]
pub enum ServerCommands {
    /// Shutdown server.
    /// You will need to restart vns daemon to running the server again.

    #[clap(value_parser)]
    Shutdown,
    /// Get server status

    #[clap(value_parser)]
    Status,
    /// Pause accepting incoming connections.
    ///May drop socket pending connection. All open connections remain active.

    #[clap(value_parser)]
    Pause,
    /// Resume accepting incoming connections.

    #[clap(value_parser)]
    Resume,
    ///  Block specific ip address to disallow using HTTP server
    #[clap(value_parser)]
    Block {
        #[clap(required = true, value_parser)]
        addresses: Vec<String>,
    },

    ///  un-Block specific ip address that was blocked and allow using HTTP server again
    #[clap(value_parser)]
    UnBlock {
        #[clap(required = true, value_parser)]
        addresses: Vec<String>,
    },
    List {
        #[clap(subcommand)]
        list: List,
    },
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Subcommand)]
pub enum List {
    Connections,
    Block,
}

impl Display for ServerCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ServerCommands::Run {} => write!(f, "run"),
            // ServerCommands::Restart => write!(f, "restart"),
            ServerCommands::Shutdown => write!(f, "shutdown"),
            ServerCommands::Status => write!(f, "status"),
            ServerCommands::Pause => write!(f, "pause"),
            ServerCommands::Resume => write!(f, "resume"),
            ServerCommands::Block { .. } => write!(f, "block"),
            ServerCommands::UnBlock { .. } => write!(f, "unblock"),
            ServerCommands::List { list } => match list {
                List::Block => write!(f, "block-list"),
                List::Connections => write!(f, "connections-list"),
            },
        }
    }
}
