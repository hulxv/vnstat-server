pub mod database;

use actix_http::header::{HeaderValue, USER_AGENT};
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use database::{InitDatabase, Keys};
use log::warn;

pub struct Auth;

impl Auth {
    pub async fn validate(
        req: ServiceRequest,
        credentials: BearerAuth,
    ) -> Result<ServiceRequest, Error> {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();

        match Keys::valid(db.conn(), credentials.token()) {
            true => Ok(req),
            _ => {
                warn!(
                    "Auth validate faild \n\t IP address: {} \n\t Peer address: {} \n\t User Agent: {:?} \n\t Authorization Token: {}",
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
}
