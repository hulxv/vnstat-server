use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]

pub struct ServerConfigs {
    pub ip: String,
    pub port: i32,
}

impl ServerConfigs {
    pub fn from(ip: &'static str, port: i32) -> Self {
        Self {
            ip: ip.to_owned(),
            port,
        }
    }
}
