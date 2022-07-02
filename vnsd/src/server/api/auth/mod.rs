pub mod database;

use actix_web::{
    dev::ServiceRequest,
    http::header::{HeaderValue, USER_AGENT},
    Error,
};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use app::Configs;
use database::{Connections, Create, InitDatabase, Keys, Statements};
use log::*;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct AuthResponse {
    pub uuid: String,
    pub key: Key,
}

impl AuthResponse {
    fn new(uuid: String, key: Key) -> Self {
        Self { key, uuid }
    }
}

#[derive(Serialize)]
pub struct Key {
    value: String,
    expires_at: String,
}
impl Key {
    fn new(value: String, expires_at: String) -> Self {
        Self { value, expires_at }
    }
}

pub enum AuthErrors {
    IncorrectPassword,
}

impl AuthErrors {
    pub fn message(&self) -> String {
        use AuthErrors::*;
        match self {
            IncorrectPassword => "Password is incorrect",
        }
        .to_string()
    }
}

pub struct Auth;

impl Auth {
    pub async fn validate(
        req: ServiceRequest,
        credentials: BearerAuth,
    ) -> Result<ServiceRequest, Error> {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();

        match Keys::is_valid(db.conn(), credentials.token()) {
            true => Ok(req),
            _ => {
                warn!(
                    "Auth validate failed \n\t IP address: {} \n\t Peer address: {} \n\t User Agent: {:?} \n\t Authorization Token: {}",
                    req.connection_info().realip_remote_addr().unwrap_or("UNKNOWN"),
                    req.connection_info().peer_addr().unwrap_or("UNKNOWN"),
                    req.headers().get(USER_AGENT).unwrap_or(&HeaderValue::from_str("UNKNOWN").unwrap()),
                    match credentials.token().len() > 30 {
                        true => credentials.token().get(..30).unwrap().to_owned() + "...",
                        _ => credentials.token().to_owned()
                    }
                );
                let config = req
                    .app_data::<Config>()
                    .map(|data| data.clone())
                    .unwrap_or_else(Default::default);
                Err(AuthenticationError::from(config).into())
            }
        }
    }

    pub fn login(
        password: &str,
        ip_addr: &str,
        user_agent: &str,
    ) -> Result<AuthResponse, AuthErrors> {
        if Configs::init().unwrap().auth().password().eq(password) {
            let db = InitDatabase::connect().unwrap();
            db.init().map_err(|e| error!("{e}")).unwrap();

            let conn = match Connections::find(db.conn(), |c| {
                c.ip_addr() == ip_addr && c.user_agent() == user_agent
            }) {
                None => Connections::new(ip_addr, user_agent)
                    .create(db.conn())
                    .unwrap(),
                Some(conn) => conn,
            };

            let key = match Keys::find(db.conn(), |k| {
                if let Some(k_conn) = k.conn(db.conn()) {
                    return k_conn == conn;
                }
                false
            }) {
                Some(k) if Keys::is_valid(db.conn(), &k.value()) => k,
                _ => Keys::generate_new_key(db.conn(), &conn.uuid())
                    .create(db.conn())
                    .unwrap(),
            };
            Ok(AuthResponse::new(
                conn.uuid(),
                Key::new(key.value(), key.expires_at().to_rfc2822()),
            ))
        } else {
            Err(AuthErrors::IncorrectPassword)
        }
    }
}
