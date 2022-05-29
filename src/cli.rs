use clap::{ArgEnum, Parser, Subcommand};
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub commands: Commands,
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
}