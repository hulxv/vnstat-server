pub mod config;
pub mod log;

pub use crate::log::*;
pub use config::*;

use anyhow::{anyhow, Result};
use std::io::{Error, ErrorKind::NotFound};


/// To get main directory
///
/// If running with root privilages or as systemd unit service:
///     STATE_DIRECTORY if it was set or "/var/lib/vnstat-server"
/// If running as regular program:
///     ~/.config/vnstat-server
///
pub struct MainDirectory;

impl MainDirectory {
    pub fn get() -> Result<String> {
        let main_dir: String;
        if std::env::var("SYSTEMD_EXEC_PID").is_ok() || std::env::var("SUDO_USER").is_ok() {
            main_dir =
                std::env::var("STATE_DIRECTORY").unwrap_or("/var/lib/vnstat-server".to_owned());
        } else {
            main_dir = match dirs::config_dir() {
                Some(path) => [
                    path.into_os_string().into_string().unwrap(),
                    "/vnstat-server".to_owned(),
                ]
                .concat(),
                None => {
                    return Err(anyhow!(Error::new(
                        NotFound,
                        "Can't find config directory".to_owned(),
                    )))
                }
            };
        }
        Ok(main_dir)
    }
}
