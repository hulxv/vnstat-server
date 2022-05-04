use crate::{
    http::response::{Response, ResponseError, ResponseStatus},
    vnstat::{
        db::models::Traffic,
        traffic::{self, is_validated_interval},
    },
};
use actix_web::{get, web, HttpResponse, Result};
use anyhow::anyhow;
use serde_json::json;
use std::io::Error as StdError;

#[get("/traffic/{interval}")]
pub async fn get_traffic(interval: web::Path<String>) -> HttpResponse {
    if !is_validated_interval(interval.clone()) {
        return HttpResponse::NotFound().json(json!(ResponseError::new()
            .code(404)
            .details("Interval isn't found.")
            .build()));
    }
    match traffic::get_traffic(interval.clone()) {
        Ok(result) => HttpResponse::Ok().json(json!(Response::new()
            .status(ResponseStatus::Success)
            .data(&result)
            .build())),
        Err(err) => HttpResponse::BadRequest().json(json!(ResponseError::new().build())),
    }
}
