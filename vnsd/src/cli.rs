use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// select pid file
    #[clap(long)]
    pub ip: Option<String>,
    /// set daemon process user
    #[clap(long)]
    pub port: Option<u16>,
}
