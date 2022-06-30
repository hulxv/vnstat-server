use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct SecurityConfigs {
    pub read_only: bool,
}

impl SecurityConfigs {
    pub fn from(read_only: bool) -> Self {
        Self { read_only }
    }
}
