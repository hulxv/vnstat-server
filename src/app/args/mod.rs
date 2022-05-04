use clap::{AppSettings, ArgEnum, Parser, Subcommand}; // use clap::{ArgEnum, Parser};
#[derive(Parser, Debug)]
// #[derive(clap::Subcommand, Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// To control your vcs server
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
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
