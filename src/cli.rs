use std::fmt::Display;

use clap::{ArgEnum, Args as ArgsMacro, Parser, Subcommand};
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
        #[clap(required = true, arg_enum)]
        command: ServerCommands,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug, Subcommand)]
pub enum ServerCommands {
    /// Shutdown server.
    /// You will need to restart vns daemon to running the server again.
    Shutdown,
    /// Get server status
    Status,
    /// Pause accepting incoming connections.
    ///May drop socket pending connection. All open connections remain active.
    Pause,
    /// Resume accepting incoming connections.
    Resume,
}

#[derive(ArgsMacro, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
/// Run youer server
struct Run {}

impl Display for ServerCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ServerCommands::Run {} => write!(f, "run"),
            // ServerCommands::Restart => write!(f, "restart"),
            ServerCommands::Shutdown => write!(f, "shutdown"),
            ServerCommands::Status => write!(f, "status"),
            ServerCommands::Pause => write!(f, "pause"),
            ServerCommands::Resume => write!(f, "resume"),
        }
    }
}
impl std::str::FromStr for ServerCommands {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // "run" => Ok(ServerCommands::Run ),
            // "restart" => Ok(ServerCommands::Restart),
            "shutdown" => Ok(ServerCommands::Shutdown),
            "status" => Ok(ServerCommands::Status),
            "pause" => Ok(ServerCommands::Pause),
            "resume" => Ok(ServerCommands::Resume),
            _ => Err("invalid"),
        }
    }
}
