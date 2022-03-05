use diesel::expression::subselect::ValidSubselect;
use dirs;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{Error, ErrorKind::Interrupted, Write},
    path::Path,
};
use toml::{map::Map, ser, value::Value};

use crate::utils::create_file;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configs {
    pub server: ServerConfigs,
    pub auth: AuthConfigs,
}

impl Configs {
    fn from(server: ServerConfigs, auth: AuthConfigs) -> Self {
        Self { server, auth }
    }

    pub fn init() -> Result<Self, Error> {
        let config_dir = match dirs::config_dir() {
            Some(path) => path.into_os_string().into_string(),
            None => panic!("Can't find \"~/.config\" directory"),
        };
        let file_path = [config_dir.unwrap(), "/vcs/vcs.config.toml".to_owned()].concat();
        let _ = match Path::new(&file_path).exists() {
            false => {
                let mut file = create_file(&file_path).unwrap();
                let configs_as_string = Self::default().to_string().unwrap();

                match file.write_all(configs_as_string.as_bytes()) {
                    Err(e) => Err(e),
                    Ok(_) => {
                        println!(
                            "Configuration file was created successfully (in {})",
                            file_path
                        );
                        Ok(())
                    }
                }
            }
            _ => Ok(()),
        };

        Ok(Self::default())
    }
    pub fn get_props(&self) -> Result<Vec<Self>, Error> {
        todo!()
    }
    pub fn default() -> Self {
        Configs::from(
            ServerConfigs::from("0.0.0.0", 8080),
            AuthConfigs::from("1234"),
        )
    }
    pub fn reset_props(&self) -> Result<(), Error> {
        todo!()
    }
    pub fn to_string(&self) -> Result<String, ser::Error> {
        toml::to_string(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ServerConfigs {
    pub ip: String,
    pub port: i32,
}

impl ServerConfigs {
    fn from(ip: &'static str, port: i32) -> Self {
        Self {
            ip: ip.to_owned(),
            port,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthConfigs {
    pub password: String,
}

impl AuthConfigs {
    fn from(password: &'static str) -> Self {
        Self {
            password: password.to_owned(),
        }
    }
}

#[test]

pub fn test_default_configs() {
    let configs_as_string = Configs::default()
        .to_string()
        .unwrap()
        .parse::<Value>()
        .unwrap();

    assert_eq!(
        configs_as_string
            .get("auth")
            .unwrap()
            .get("password")
            .unwrap()
            .as_str(),
        Some("1234")
    );
    assert_eq!(
        configs_as_string
            .get("server")
            .unwrap()
            .get("ip")
            .unwrap()
            .as_str(),
        Some("0.0.0.0")
    );
    assert_eq!(
        configs_as_string
            .get("server")
            .unwrap()
            .get("port")
            .unwrap()
            .as_integer(),
        Some(8080)
    );
    assert!(true)
}

#[test]
pub fn build_configuration_file() {
    Configs::init().unwrap();

    assert!(true)
}
