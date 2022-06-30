use actix_web::{get, post, HttpResponse};

use crate::http::response::*;
use libvnstat::VnStat;
use log::error;
use serde_json::json;

#[get("/daemon")]
pub async fn get_daemon_status() -> HttpResponse {
    match VnStat.daemon().is_active() {
        Ok(is_active) => HttpResponse::Ok().json(
            Response::new()
                .status(ResponseStatus::Success)
                .data(json!({ "is_active": is_active }))
                .build(),
        ),
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError().json(ResponseError::new().build())
        }
    }
}

#[post("/daemon/restart")]
pub async fn restart_daemon() -> HttpResponse {
    match VnStat.daemon().restart() {
        Ok(exit_status) => match exit_status.success() {
            true => HttpResponse::Ok().json(
                Response::new()
                    .status(ResponseStatus::Success)
                    .data(json!({"details":"vnStatD restarted successfully"}))
                    .build(),
            ),
            false => HttpResponse::InternalServerError().json(
                ResponseError::new()
                    .code(exit_status.code().unwrap() as u32)
                    .details("Cannot restarting vnStatD")
                    .build(),
            ),
        },
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError().json(ResponseError::new().build())
        }
    }
}
#[post("/daemon/stop")]
pub async fn stop_daemon() -> HttpResponse {
    match VnStat.daemon().stop() {
        Ok(exit_status) => match exit_status.success() {
            true => HttpResponse::Ok().json(
                Response::new()
                    .status(ResponseStatus::Success)
                    .data(json!({"details":"vnStatD stopped successfully"}))
                    .build(),
            ),
            false => HttpResponse::InternalServerError().json(
                ResponseError::new()
                    .code(exit_status.code().unwrap() as u32)
                    .details("Cannot stopping vnStatD")
                    .build(),
            ),
        },
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError().json(ResponseError::new().build())
        }
    }
}
