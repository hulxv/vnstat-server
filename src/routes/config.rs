use crate::{http::response::*, vnstat::config::VnStatConfigs};
use actix_web::{get, post, web, HttpResponse, Result};
use serde_json::json;
use std::rc::Rc;

#[get("/config")]
pub async fn get_config() -> HttpResponse {
    match VnStatConfigs::default().get_props() {
        Err(err) => HttpResponse::InternalServerError().json(ResponseError::new().build()),
        Ok(result) => HttpResponse::Ok().json(json!(result)),
    }
}

#[post("/config")]
pub async fn edit_config(key: String, value: String) -> HttpResponse {
    todo!()
}
