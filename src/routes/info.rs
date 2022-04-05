use crate::{
    http::response::{Response, ResponseError, ResponseStatus},
    vnstat::db::{models::info::Info, Database},
};
use actix_web::{get, web, HttpResponse, Responder, Result};
use serde_json::json;

#[get("/info")]
pub async fn get_info() -> HttpResponse {
    match Database::default()
        .unwrap()
        .connect()
        .unwrap()
        .select_table::<Info>("info".to_owned())
    {
        Ok(result) => HttpResponse::Ok().json(json!(Response::new()
            .status(ResponseStatus::Success)
            .data(&result)
            .build())),
        Err(err) => {
            HttpResponse::InternalServerError().json(json!(ResponseError::new().details("There's an internal server error happend, Please check 'vcs' logs for more details").build()))
        }
    }
}
