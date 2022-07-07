use crate::api::auth::Auth;
use crate::http::response::*;
use actix_web::{
    dev::ConnectionInfo, http::header::USER_AGENT, post, web, HttpRequest, HttpResponse,
};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Payload {
    password: String,
}
#[post("/login")]
pub async fn login(
    payload: web::Json<Payload>,
    conn: ConnectionInfo,
    req: HttpRequest,
) -> HttpResponse {
    match Auth::login(
        &payload.password,
        conn.realip_remote_addr().unwrap(),
        req.headers().get(USER_AGENT).unwrap().to_str().unwrap(),
    ) {
        Ok(result) => HttpResponse::Ok().json(
            Response::new()
                .status(ResponseStatus::Success)
                .data(&result)
                .build(),
        ),
        Err(err) => HttpResponse::Unauthorized().json(
            ResponseError::new()
                .code(401)
                .details(err.message().as_str())
                .build(),
        ),
    }
}
