use actix_web::HttpResponse;

use crate::http::response::ResponseError;

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(ResponseError::new().code(404).details("service not found."))
}
