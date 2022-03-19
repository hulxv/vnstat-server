use actix_web::{get, web, Error as ActixError, HttpResponse, Result};
use serde_json::json;

use crate::{
    http::response::{Response, ResponseError},
    vnstat::{
        db::models::traffic::Traffic,
        traffic::{self, is_validated_interval},
    },
};
use std::io::Error as StdError;

#[get("/traffic/{interval}")]
pub async fn get_traffic(interval: web::Path<String>) -> Result<HttpResponse> {
    if !is_validated_interval(interval.to_string()) {
        return Ok(
            HttpResponse::BadRequest().json(json!(ResponseError::new_response(
                "invalid interval".to_owned(),
                404
            ))),
        );
    }
    match traffic::get_traffic(interval.to_string()) {
        Ok(result) => {
            Ok(HttpResponse::Ok().json(json!(Response::<Vec<Traffic>>::new("success", result))))
        }
        Err(err) => Ok(HttpResponse::BadRequest()
            .json(json!(ResponseError::new_response(err.to_string(), 502)))),
    }
}
