use derivative::Derivative;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Derivative, Debug, Clone)]
#[derivative(Default)]
pub struct VnstatConfigs {
    #[derivative(Default(value = "Some(\"/etc/vnstat.conf\".to_owned())"))]
    config_file: Option<String>,
}

impl VnstatConfigs {
    pub fn from(config_file: &'static str) -> Self {
        Self {
            config_file: Some(config_file.to_owned()),
        }
    }
    pub fn config_file(&self) -> String {
        self.config_file
            .clone()
            .unwrap_or("/etc/vnstat.conf".to_owned())
    }
}
