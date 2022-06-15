use crate::http::response::{Response, ResponseError, ResponseStatus};
use actix_web::{get, web, HttpResponse, Responder, Result};
use libvnstat::VnStat;
use serde_json::json;

#[get("/info")]
pub async fn get_info() -> HttpResponse {
    match VnStat.info().get()
    {
        Ok(result) => HttpResponse::Ok().json(json!(Response::new()
            .status(ResponseStatus::Success)
            .data(&result)
            .build())),
        Err(err) => {
            HttpResponse::InternalServerError().json(json!(ResponseError::new().details("There's an internal server error happend, Please check 'vns' logs for more details").build()))
        }
    }
}
