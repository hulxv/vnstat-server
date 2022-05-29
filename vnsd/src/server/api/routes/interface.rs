use std::io::Error as StdError;

use serde_json::json;

use crate::http::response::{Response, ResponseError, ResponseStatus};
use actix_web::{get, HttpResponse, Result};
use libvnstat::VnStat;
#[get("/interface")]
pub async fn get_interface() -> HttpResponse {
    match VnStat.interface().get() {
        Ok(result) => HttpResponse::Ok().json(json!(Response::new()
            .status(ResponseStatus::Success)
            .data(&result)
            .build())),
        Err(err) => HttpResponse::InternalServerError().json(json!(ResponseError::new().build())),
    }
}
