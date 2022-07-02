use derivative::Derivative;
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Derivative, Clone)]
#[derivative(Default)]
pub struct SecurityConfigs {
    #[derivative(Default(value = "Some(true)"))]
    read_only: Option<bool>,
}

impl SecurityConfigs {
    pub fn from(read_only: bool) -> Self {
        Self {
            read_only: Some(read_only),
        }
    }

    pub fn read_only(&self) -> bool {
        self.read_only.clone().unwrap_or(true)
    }
}
