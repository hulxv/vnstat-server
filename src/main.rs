use std::{
    io::{self, Write},
    process, thread,
};

use vcs::*;

pub async fn cli_runner() {
    println!(
        "VCS is running, Use it by enter commands or type \".help\" to show available commands or type \"q\" to quit."
    );
    loop {
        print!("~> ",);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().to_lowercase().as_str() {
                "run" => {
                    println!("Running server...");
                    thread::spawn(|| {
                        println!("...",);
                        run_server().unwrap()
                    });
                }
                "exit" | "q" | "quit" => {
                    println!("Program has been terminated");
                    process::exit(1)
                }
                _ => (),
            },
            Err(err) => panic!("{}", err),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello !");
    tokio::spawn(async {
        cli_runner().await;
    });
    Ok(())
}
