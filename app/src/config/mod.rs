use anyhow::{anyhow, Result};
use derivative::Derivative;
use dirs;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind::NotFound},
};
use toml::value::Value;
use utils::file::File;

pub mod auth;
pub mod security;
pub mod server;
pub mod vnstat;

use self::{auth::*, security::*, server::*, vnstat::*};

#[derive(Serialize, Deserialize, Debug, Derivative)]
pub struct Configs {
    server: Option<ServerConfigs>,

    auth: Option<AuthConfigs>,

    vnstat: Option<VnstatConfigs>,
    security: Option<SecurityConfigs>,
}

impl Configs {
    pub fn from(
        server: Option<ServerConfigs>,
        auth: Option<AuthConfigs>,
        vnstat: Option<VnstatConfigs>,
        security: Option<SecurityConfigs>,
    ) -> Self {
        Self {
            server,
            auth,
            vnstat,
            security,
        }
    }

    pub fn default() -> Self {
        Configs::from(
            Some(ServerConfigs::default()),
            Some(AuthConfigs::default()),
            Some(VnstatConfigs::default()),
            Some(SecurityConfigs::default()),
        )
    }

    pub fn init() -> Result<Self> {
        let path = Self::get_file_path()?;
        println!("{path}");
        let _ = match File::new(path.clone()).exists() {
            false => File::new(path.clone()).create(Self::default().to_string()?),
            _ => Ok(()),
        };

        Ok(toml::from_str(
            fs::read_to_string(Self::get_file_path()?)?.as_str(),
        )?)
    }

    pub fn reset() -> Result<()> {
        let path = Self::get_file_path()?;
        fs::remove_file(&path)?;
        File::new(path.clone()).create(Self::default().to_string()?)?;
        Ok(())
    }

    pub fn to_string(&self) -> Result<String> {
        Ok(toml::to_string(self)?)
    }

    /// Convert 'query' to keys to get 'toml::value::Value'
    ///
    /// ### Example
    /// #### Toml content:
    /// ```
    ///   [foo]
    ///   boo = "hi"
    /// ```
    /// print!("{}", get("foo.boo"))
    ///
    /// result : "hi"
    ///
    pub fn get(&self, query: &str) -> Option<Value> {
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
                _ => next_val,
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
        Ok([config_dir.unwrap(), "/vns/vns.config.toml".to_owned()].concat())
    }

    pub fn vnstat(&self) -> VnstatConfigs {
        self.vnstat.clone().unwrap_or_default()
    }
    pub fn server(&self) -> ServerConfigs {
        self.server.clone().unwrap_or_default()
    }
    pub fn security(&self) -> SecurityConfigs {
        self.security.clone().unwrap_or_default()
    }
    pub fn auth(&self) -> AuthConfigs {
        self.auth.clone().unwrap_or_default()
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
    println!(
        "{}",
        configs.get("server.address").unwrap().as_str().unwrap()
    );
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
