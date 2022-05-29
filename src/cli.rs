use std::fmt::Display;

use clap::{ArgEnum, Parser, Subcommand};
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
pub enum ServerCommands {
    Run,
    Shutdown,
    Restart,
}

impl Display for ServerCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerCommands::Run => write!(f, "run"),
            ServerCommands::Restart => write!(f, "restart"),
            ServerCommands::Shutdown => write!(f, "shutdown"),
        }
    }
}
