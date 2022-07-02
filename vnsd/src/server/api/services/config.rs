use crate::http::response::*;
use actix_web::{get, put, web, HttpResponse};
use app::config::Configs;
use libvnstat::VnStat;
use log::{error, info};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

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

#[derive(Deserialize, Serialize, Clone)]
pub struct Payload {
    pub key: String,
    pub value: String,
}

#[put("/config")]
pub async fn edit_config(payload: web::Json<Payload>) -> HttpResponse {
    if Configs::init().unwrap().security().read_only() {
        return HttpResponse::Forbidden().json(
            ResponseError::new()
                .code(403)
                .details("Cannot do this operation: read-only mode was activated.")
                .build(),
        );
    }
    match VnStat
        .config()
        .set_prop(&payload.clone().key, &payload.clone().value)
        .await
    {
        Ok(exit_status) => {
            info!("{exit_status}");
            HttpResponse::Ok().json(
                Response::new()
                    .status(ResponseStatus::Success)
                    .data(json!({ payload.clone().key: payload.clone().value }))
                    .build(),
            )
        }
        Err(err) => {
            error!("{err}");
            HttpResponse::InternalServerError().json(ResponseError::new().build())
        }
    }
}
