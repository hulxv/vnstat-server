use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    path::Path,
    process::{Child, Command as StdCommand, Stdio},
};

pub trait CommandProps {
    fn get_args(&self) -> Vec<String>;
    fn get_program(&self) -> String;
    fn get_envs(&self) -> Vec<(String, String)>;
    fn get_current_dir(&self) -> &Path;
    fn status(&self);
}

pub trait CommandOutput {
    fn stdout(&self) -> Result<String>;
    fn stderr(&self) -> Result<String>;
}

#[derive(Debug)]
pub struct Command {
    command: StdCommand,
    program: String,
    args: Vec<String>,
}

impl Command {
    pub fn new(cmd: &str) -> Self {
        let pattern = Regex::new(r#"[^\s"']+|"([^"]*)"|'([^']*)'"#).unwrap();
        let matches: Vec<String> = pattern
            .find_iter(cmd.clone())
            .map(|m| m.as_str().to_string())
            .collect();
        let mut command = StdCommand::new("sh");
        command.arg("-c");
        command.arg(cmd);
        println!("{:?}", command);
        Command {
            command: command,
            program: matches[0].to_owned(),
            args: matches[1..].to_vec(),
        }
    }

    pub fn needs_root(&self) -> Result<bool> {
        todo!()
    }
    pub fn exec(&self) -> Result<Child> {
        Ok(StdCommand::new(self.program.clone())
            .args(&self.args)
            .stdout(Stdio::piped())
            .spawn()?)
    }
}

impl CommandOutput for Command {
    fn stdout(&self) -> Result<String> {
        Ok(String::from_utf8(
            StdCommand::new(self.program.clone())
                .args(&self.args)
                .stdout(Stdio::piped())
                .output()?
                .stdout,
        )?)
    }
    fn stderr(&self) -> Result<String> {
        Ok(String::from_utf8(
            StdCommand::new(self.program.clone())
                .args(&self.args)
                .stdout(Stdio::piped())
                .output()?
                .stderr,
        )?)
    }
}

impl CommandProps for Command {
    fn get_program(&self) -> String {
        self.program.clone()
    }

    fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }
    fn get_envs(&self) -> Vec<(String, String)> {
        self.command
            .get_envs()
            .map(|(_0, _1)| {
                (
                    _0.to_string_lossy().to_string(),
                    _1.unwrap().to_string_lossy().to_string(),
                )
            })
            .collect::<Vec<(String, String)>>()
    }
    fn get_current_dir(&self) -> &Path {
        &self.command.get_current_dir().unwrap()
    }
    fn status(&self) {
        todo!()
    }
}

#[test]
fn test_create_new_command() -> Result<()> {
    let with_command = Command::new("echo \"Hello world!\"").stdout()?;
    let with_std_command = StdCommand::new("echo").arg("\"Hello world!\"").output()?;
    assert_eq!(
        with_command.as_str(),
        &String::from_utf8_lossy(with_std_command.stdout.as_slice()),
        "Hello world!"
    );

    Ok(())
}
