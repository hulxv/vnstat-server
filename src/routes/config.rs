use crate::{http::response::*, vnstat::config::VnStatConfigs};
use actix_web::{get, post, web, HttpResponse, Result};
use serde_json::{json, to_string_pretty};

#[get("/config")]
pub async fn get_config() -> HttpResponse {
    match VnStatConfigs::props() {
        Err(err) => HttpResponse::InternalServerError().json(ResponseError::new().build()),
        Ok(result) => HttpResponse::Ok().json(json!(Response::new()
            .status(ResponseStatus::Success)
            .data(&result)
            .build())),
    }
}

#[post("/config")]
pub async fn edit_config(key: String, value: String) -> HttpResponse {
    todo!()
}
