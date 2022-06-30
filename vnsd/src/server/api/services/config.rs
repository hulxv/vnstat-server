use crate::http::response::*;
use actix_web::{get, post, HttpResponse};
use libvnstat::VnStat;
use log::error;

#[get("/config")]
pub async fn get_config() -> HttpResponse {
    match VnStat.config().get_props() {
        Ok(result) => HttpResponse::Ok().json(
            Response::new()
                .status(ResponseStatus::Success)
                .data(&result)
                .build(),
        ),
        Err(err) => {
            error!("{err}");
            HttpResponse::InternalServerError().json(ResponseError::new().build())
        }
    }
}

#[post("/config")]
pub async fn edit_config(key: String, value: String) -> HttpResponse {
    todo!()
}
