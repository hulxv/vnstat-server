use clap::{AppSettings, ArgEnum, Parser, Subcommand}; // use clap::{ArgEnum, Parser};
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// select pid file
    #[clap(long, short)]
    pub pid_file: Option<String>,
    /// set daemon process user
    #[clap(long, short)]
    pub user: Option<String>,
    /// set daemon process group
    #[clap(long, short)]
    pub group: Option<String>,
    /// run in foreground
    #[clap(long, short)]
    pub foreground: bool,
}
