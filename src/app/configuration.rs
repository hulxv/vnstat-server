use anyhow::{anyhow, Result};
use dirs;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{
        Error,
        ErrorKind::{Interrupted, NotFound},
        Write,
    },
    path::Path,
};
use toml::{ser, value::Value};

use crate::utils::create_file;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configs {
    pub server: ServerConfigs,
    pub auth: AuthConfigs,
}

impl Configs {
    pub fn from(server: ServerConfigs, auth: AuthConfigs) -> Self {
        Self { server, auth }
    }

    pub fn default() -> Self {
        Configs::from(
            ServerConfigs::from("0.0.0.0", 8080),
            AuthConfigs::from("1234"),
        )
    }

    pub fn init() -> Result<Self> {
        let _ = match Path::new(&Self::get_file_path()?).exists() {
            false => Self::build_from(Self::default().to_string()?),
            _ => Ok(()),
        };

        Ok(toml::from_str(
            fs::read_to_string(Self::get_file_path()?)?.as_str(),
        )?)
    }

    pub fn build_from(content: String) -> Result<()> {
        let mut file = create_file(&Self::get_file_path()?)?;
        match file.write_all(content.as_bytes()) {
            Err(err) => Err(anyhow!(err)),
            Ok(_) => {
                println!(
                    "Configuration file was created successfully (in {})",
                    &Self::get_file_path()?
                );
                Ok(())
            }
        }
    }

    pub fn reset(&self) -> Result<()> {
        match fs::remove_file(Self::get_file_path()?) {
            Err(err) => return Err(anyhow!(err)),
            _ => (),
        };
        let mut file = create_file(&Self::get_file_path()?)?;
        match file.write_all(Self::default().to_string()?.as_bytes()) {
            Err(err) => return Err(anyhow!(err)),
            Ok(_) => {
                println!("Configuration file was reset successfully",);
            }
        };
        Ok(())
    }
    pub fn to_string(&self) -> std::result::Result<String, ser::Error> {
        Ok(toml::to_string(self)?)
    }

    pub fn get(&self, query: &str) -> Option<Value> {
        /*
         * Convert 'query' to keys to get 'toml::value::Value'
         *
         * - Example
         *  - Toml content
         *     '
         *         [foo]
         *         boo = "hi"
         *     '
         * query = "foo.boo" ===> "hi"
         */
        let keys: Vec<&str> = query.split(".").map(|e| e.trim()).collect::<Vec<&str>>();

        // * Convert Configs to string and parse it to 'toml::value::Value'
        let configs_as_string = self.to_string().unwrap().as_str().parse::<Value>().unwrap();

        let mut value: Option<&Value> = None;

        for key in keys.iter() {
            let next_val = if let None = value {
                configs_as_string.get(key)
            } else {
                value?.get(key)
            };
            if let None = next_val {
                return None;
            }
            value = match value {
                // * value will be 'Option::None' in first loop
                None => configs_as_string.get(key),

                Some(val) => next_val,
            };
        }
        Some(value?.to_owned())
    }

    pub fn get_file_path() -> Result<String> {
        let config_dir = match dirs::config_dir() {
            Some(path) => path.into_os_string().into_string(),
            None => {
                return Err(anyhow!(Error::new(
                    NotFound,
                    "Can't find \"~/.config\" directory".to_owned(),
                )))
            }
        };
        Ok([config_dir.unwrap(), "/vcs/vcs.config.toml".to_owned()].concat())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    let configs = Configs::default();

    assert_eq!(configs.get("auth.password").unwrap().as_str(), Some("1234"));
    assert_eq!(configs.get("server.ip").unwrap().as_str(), Some("0.0.0.0"));
    assert_eq!(configs.get("server.port").unwrap().as_integer(), Some(8080));
    assert!(true)
}
#[test]
pub fn test_get_prop_from_configs() {
    let configs = Configs::default();

    assert_eq!(configs.get("auth.password").unwrap().as_str(), Some("1234"));
    assert_eq!(configs.get("server.ip").unwrap().as_str(), Some("0.0.0.0"));
    assert_eq!(configs.get("server.port").unwrap().as_integer(), Some(8080));
    assert_eq!(configs.get("foo.boo"), None);
    assert!(true)
}

#[test]
pub fn test_build_configuration_file() {
    Configs::init().unwrap();

    assert!(true)
}
#[test]
fn test_get_configuration_file_path() {
    Configs::get_file_path().unwrap();
    assert!(true)
}
