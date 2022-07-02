use derivative::Derivative;
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Derivative, Clone)]
#[derivative(Default)]

pub struct ServerConfigs {
    #[derivative(Default(value = "Some(\"0.0.0.0\".to_string())"))]
    ip: Option<String>,

    #[derivative(Default(value = "Some(8080)"))]
    port: Option<u16>,
}

impl ServerConfigs {
    pub fn from(ip: &'static str, port: u16) -> Self {
        Self {
            ip: Some(ip.to_owned()),
            port: Some(port),
        }
    }

    pub fn ip(&self) -> String {
        self.ip.clone().unwrap_or("0.0.0.0".to_owned())
    }
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(8080)
    }
}
