use std::{
    io::{self, Write},
    process, thread,
};

use vcs::*;

pub fn cli_runner() {
    println!(
        "VCS is running, Use it by enter commands or type \".help\" to show available commands or type \"q\" to quit."
    );
    loop {
        print!("vcs ~> ",);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().to_lowercase().as_str() {
                "run" => {
                    println!("Running server...");
                    thread::spawn(|| run_server().unwrap());
                }
                "exit" | "q" | "quit" => {
                    println!("Program has been terminated");
                    process::exit(1)
                }
                _ => (),
            },
            Err(err) => eprintln!("{}", err),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::task::spawn_blocking(|| {
        cli_runner();
    })
    .await?;
    Ok(())
}
