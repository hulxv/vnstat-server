use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct VnstatConfigs {
    pub config_file: String,
}

impl VnstatConfigs {
    pub fn from(config_file: &'static str) -> Self {
        Self {
            config_file: config_file.to_owned(),
        }
    }
}
