use clap::{clap_derive::ArgEnum, Parser, Subcommand};
use std::fmt::Display;
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "
a Utility used by the end-user to control in vnStat HTTP server efficiently and easily by communicate with vnsd by unix-socket.
"
)]

pub struct Args {
    #[clap(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    //     #[clap(override_help = "
    //  To controlling in your vns HTTP server.

    // USAGE:
    //     vns server <COMMAND>

    // OPTIONS:
    //     -h, --help    Print help information

    // COMMANDS:
    //     help
    //         Print this message or the help of the given subcommand(s)
    //     list <connections|block>
    //         get list of blocks or connections in vns HTTP server.
    //     pause
    //         Pause accepting incoming connections. May drop socket pending connection. All
    //         open connections remain active.

    //     resume
    //         Resume accepting incoming connections.

    //     shutdown
    //         Shutdown server. You will need to restart vns daemon to running the server again.

    //     status
    //         Get vns HTTP server status.

    //     block <IP_ADDRESSES>...
    //         Block specific ip address to disallow using HTTP server.

    //     un-block <IP_ADDRESSES>...
    //         un-Block specific ip address that was blocked and allow using HTTP server again.
    // ")]
    /// To controlling in your vns HTTP server.
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
    //     #[clap(override_help = "
    // USAGE:
    //     vns server list <connections | block>

    // OPTIONS:
    //     -h, --help    Print help information
    // ")]
    List {
        #[clap(arg_enum)]
        list: List,
    },
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ArgEnum)]
pub enum List {
    Connections,
    Block,
}

impl Display for ServerCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
