use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthConfigs {
    pub password: String,
    pub key_expire_duration: i64,
}

impl AuthConfigs {
    pub fn from(password: &'static str, key_expire_duration: i64) -> Self {
        Self {
            password: password.to_owned(),
            key_expire_duration: key_expire_duration.to_owned(),
        }
    }
}
