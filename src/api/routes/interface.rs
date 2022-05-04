use std::io::Error as StdError;

use serde_json::json;

use crate::{
    http::response::{Response, ResponseError, ResponseStatus},
    vnstat::db::{models::Interface, Database},
};
use actix_web::{get, HttpResponse, Result};
#[get("/interface")]
pub async fn get_interfaces() -> HttpResponse {
    match Database::default()
        .unwrap()
        .connect()
        .unwrap()
        .select_table::<Interface>("interface")
    {
        Ok(result) => HttpResponse::Ok().json(json!(Response::new()
            .status(ResponseStatus::Success)
            .data(&result)
            .build())),
        Err(err) => HttpResponse::InternalServerError().json(json!(ResponseError::new().build())),
    }
}
