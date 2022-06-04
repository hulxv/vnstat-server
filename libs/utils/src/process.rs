use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    path::Path,
    process::{Child, Command as StdCommand, Stdio},
};

trait CommandProps {
    fn get_args(&self) -> &Vec<&str>;
    fn get_program(&self) -> &str;
    fn get_envs(&self) -> Vec<(&str, &str)>;
    fn get_current_dir(&self) -> &Path;
    fn status(&self);
}

trait CommandIO {
    fn stdout(&self) -> Result<String>;
    fn stdin(&self) -> Result<String>;
    fn stderr(&self) -> Result<String>;
}

#[derive(Debug)]
pub struct Command {
    command: StdCommand,
    program: &'static str,
    args: Vec<&'static str>,
}

impl Command {
    pub fn new<'a>(cmd: &'static str) -> Self {
        let pattern = Regex::new(r#"[^\s"']+|"([^"]*)"|'([^']*)'"#).unwrap();
        let matches: Vec<&str> = pattern.find_iter(cmd).map(|m| m.as_str()).collect();
        let mut command = StdCommand::new("sh");
        command.arg("-c");
        command.arg(cmd);
        Command {
            command: command,
            program: matches[0],
            args: matches[1..].to_vec(),
        }
    }

    pub fn needs_root(&self) -> Result<bool> {
        todo!()
    }
    pub fn exec(&self) -> Result<Child> {
        match StdCommand::new(self.program)
            .args(&self.args)
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(err) => Err(anyhow!(err)),
            Ok(child) => Ok(child),
        }
    }
}

impl CommandIO for Command {
    fn stdout(&self) -> Result<String> {
        match StdCommand::new(self.program)
            .args(&self.args)
            .stdout(Stdio::piped())
            .output()
        {
            Err(err) => Err(anyhow!(err)),
            Ok(output) => Ok(String::from_utf8(output.stdout)?),
        }
    }
    fn stderr(&self) -> Result<String> {
        todo!()
    }
    fn stdin(&self) -> Result<String> {
        todo!()
    }
}

impl CommandProps for Command {
    fn get_program(&self) -> &str {
        self.program
    }

    fn get_args(&self) -> &Vec<&str> {
        &self.args
    }
    fn get_envs(&self) -> Vec<(&str, &str)> {
        self.command
            .get_envs()
            .map(|(_0, _1)| (_0.to_str().unwrap(), _1.unwrap().to_str().unwrap()))
            .collect::<Vec<(&str, &str)>>()
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
