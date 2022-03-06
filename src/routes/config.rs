use actix_web::{get, post, web, HttpResponse, Result};

use crate::http::response::*;

#[get("/daemon")]
pub async fn get_daemon_status() -> Result<HttpResponse> {
    todo!()
}

#[post("/daemon")]
pub async fn change_daemon_status(status: String) -> Result<HttpResponse> {
    todo!()
}
