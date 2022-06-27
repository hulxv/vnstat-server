use actix_web::{get, post, HttpResponse};

use crate::http::response::*;

#[get("/daemon")]
pub async fn get_daemon_status() -> HttpResponse {
    todo!()
}

#[post("/daemon")]
pub async fn change_daemon_status(status: String) -> HttpResponse {
    todo!()
}
