use derivative::Derivative;
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Derivative, Clone)]
#[derivative(Default)]
pub struct AuthConfigs {
    #[derivative(Default(value = "Some(\"password\".to_string())"))]
    password: Option<String>,

    #[derivative(Default(value = "Some(2)"))]
    key_expire_duration: Option<i64>,
}

impl AuthConfigs {
    pub fn from(password: &'static str, key_expire_duration: i64) -> Self {
        Self {
            password: Some(password.to_owned()),
            key_expire_duration: Some(key_expire_duration.to_owned()),
        }
    }

    pub fn password(&self) -> String {
        self.password.clone().unwrap_or("password".to_owned())
    }
    pub fn key_expire_duration(&self) -> i64 {
        self.key_expire_duration.unwrap_or(2)
    }
}
