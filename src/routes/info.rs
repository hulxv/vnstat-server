use crate::{
    http::response::*,
    vnstat::db::{models::info::Info, Database},
};
use actix_web::{get, web, HttpResponse, Result};
use serde_json::json;

#[get("/info")]
pub async fn get_info() -> Result<HttpResponse> {
    match Database::default()?
        .connect()?
        .select_table::<Info>("info".to_owned())
    {
        Ok(result) => {
            Ok(HttpResponse::Ok().json(json!(Response::<Vec<Info>>::new("success", result))))
        }
        Err(err) => Ok(
            HttpResponse::BadRequest().json(json!(ResponseError::new_response(
                format!("{err:?}").as_str(),
                502
            ))),
        ),
    }
}
