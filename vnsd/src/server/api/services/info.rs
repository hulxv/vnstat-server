use crate::http::response::{Response, ResponseError, ResponseStatus};
use actix_web::{get, HttpResponse};
use libvnstat::VnStat;
use log::error;

#[get("/info")]
pub async fn get_info() -> HttpResponse {
    match VnStat.info().get() {
        Ok(result) => HttpResponse::Ok().json(
            Response::new()
                .status(ResponseStatus::Success)
                .data(&result)
                .build(),
        ),

        Err(err) => {
            error!("{err}");

            HttpResponse::InternalServerError().json(ResponseError::new().details("There's an internal server error happend, Please check 'vns' logs for more details").build())
        }
    }
}
