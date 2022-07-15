use crate::http::response::{Response, ResponseError, ResponseStatus};
use actix_web::{get, HttpResponse};
use libvnstat::{db::models::Info, VnStat};
use log::error;

#[get("/info")]
pub async fn get_info() -> HttpResponse {
    match VnStat.info().get() {
        Ok(mut result) => {
            result.push(Info {
                id: result.len() as i32 + 1,
                name: "vns-version".to_owned(),
                value: env!("CARGO_PKG_VERSION").to_owned(),
            });
            HttpResponse::Ok().json(
                Response::new()
                    .status(ResponseStatus::Success)
                    .data(&result)
                    .build(),
            )
        }

        Err(err) => {
            error!("{err}");
            HttpResponse::InternalServerError().json(ResponseError::new().build())
        }
    }
}
