use actix_web::HttpResponse;
use serde_json::json;

use crate::http::response::ResponseError;

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(json!(ResponseError::new()
        .code(404)
        .details("service not found.")))
}
