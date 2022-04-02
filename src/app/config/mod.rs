use crate::utils::file::File;
use anyhow::{anyhow, Result};
use dirs;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs,
    io::{
        Error,
        ErrorKind::{Interrupted, NotFound},
    },
};
use toml::{de, ser, value::Value};

pub mod auth;
pub mod server;
pub mod vnstat;

use self::{auth::AuthConfigs, server::ServerConfigs, vnstat::VnstatConfigs};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configs {
    pub server: ServerConfigs,
    pub auth: AuthConfigs,
    pub vnstat: VnstatConfigs,
}

impl Configs {
    pub fn from(server: ServerConfigs, auth: AuthConfigs, vnstat: VnstatConfigs) -> Self {
        Self {
            server,
            auth,
            vnstat,
        }
    }

    pub fn is_any_prop_missing() -> bool {
        toml::from_str::<Self>(
            fs::read_to_string(Self::get_file_path().unwrap())
                .unwrap()
                .as_str(),
        )
        .is_err()
    }

    pub fn default() -> Self {
        Configs::from(
            ServerConfigs::from("0.0.0.0", 8080),
            AuthConfigs::from("1234"),
            VnstatConfigs::from("/etc/vnstat.conf"),
        )
    }

    pub fn init() -> Result<Self> {
        let path = Self::get_file_path()?;
        let _ = match File::new(path.clone()).exists() {
            false => File::new(path.clone()).create(Self::default().to_string()?),
            _ => Ok(()),
        };

        match Self::is_any_prop_missing() {
            true => {
                println!("Some properties are missing, so we need to reset the configuration file so that it won't do any harm.");
                match Self::reset() {
                    Err(e) => return Err(anyhow!("operation failed: {}", e)),
                    Ok(_) => {
                        println!(
                            "[{}]Configuration file was reset successfully.",
                            Self::get_file_path()?
                        );
                    }
                }
            }
            _ => (),
        }
        Ok(toml::from_str(
            fs::read_to_string(Self::get_file_path()?)?.as_str(),
        )?)
    }

    pub fn reset() -> Result<()> {
        let path = Self::get_file_path()?;
        match fs::remove_file(&path) {
            Err(err) => return Err(anyhow!(err)),
            _ => (),
        };
        match File::new(path.clone()).create(Self::default().to_string()?) {
            Err(err) => return Err(anyhow!(err)),
            Ok(_) => Ok(()),
        }
    }

    pub fn to_string(&self) -> Result<String> {
        match toml::to_string(self) {
            Ok(r) => Ok(r),
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub fn get(&self, query: &str) -> Option<Value> {
        /*
         * Convert 'query' to keys to get 'toml::value::Value'
         *
         * - Example
         *  - Toml content:
         *     '
         *         [foo]
         *         boo = "hi"
         *     '
         * print!("{}", get("foo.boo"))
         *
         * result : "hi"
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

#[test]

pub fn test_default_configs() {
    let configs = Configs::default();

    assert_eq!(configs.get("auth.password").unwrap().as_str(), Some("1234"));
    assert_eq!(configs.get("server.ip").unwrap().as_str(), Some("0.0.0.0"));
    assert_eq!(configs.get("server.port").unwrap().as_integer(), Some(8080));
    assert_eq!(
        configs.get("vnstat.config_file").unwrap().as_str(),
        Some("/etc/vnstat.conf")
    );
    assert!(true)
}
#[test]
pub fn test_get_prop_from_configs() {
    let configs = Configs::default();

    assert_eq!(configs.get("auth.password").unwrap().as_str(), Some("1234"));
    assert_eq!(configs.get("server.ip").unwrap().as_str(), Some("0.0.0.0"));
    assert_eq!(configs.get("server.port").unwrap().as_integer(), Some(8080));
    assert_eq!(
        configs.get("vnstat.config_file").unwrap().as_str(),
        Some("/etc/vnstat.conf")
    );
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
