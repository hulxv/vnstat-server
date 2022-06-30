use log::error;

use crate::http::response::{Response, ResponseError, ResponseStatus};
use actix_web::{get, HttpResponse};
use libvnstat::VnStat;
#[get("/interface")]
pub async fn get_interface() -> HttpResponse {
    match VnStat.interface().get() {
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
