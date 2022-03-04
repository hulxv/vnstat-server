use serde_json::json;

use crate::{
    http::response::*,
    vnstat::db::{models::interface::Interface, Database},
};
use actix_web::{get, HttpResponse, Result};
#[get("/interface")]
pub async fn get_interfaces() -> Result<HttpResponse> {
    match Database::default()?
        .connect()?
        .select_table::<Interface>("interface".to_owned())
    {
        Ok(result) => {
            Ok(HttpResponse::Ok().json(json!(Response::<Vec<Interface>>::new("success", result))))
        }
        Err(err) => Ok(
            HttpResponse::BadRequest().json(json!(ResponseError::new_response(
                format!("{err:?}").as_str(),
                502
            ))),
        ),
    }
}
