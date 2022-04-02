use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthConfigs {
    pub password: String,
}

impl AuthConfigs {
    pub fn from(password: &'static str) -> Self {
        Self {
            password: password.to_owned(),
        }
    }
}
