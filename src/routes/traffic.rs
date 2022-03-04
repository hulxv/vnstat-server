use actix_web::{get, web, HttpResponse, Result};
use serde_json::json;

use crate::{
    http::response::*,
    vnstat::db::{models::traffic::Traffic, Database},
};

#[get("/traffic/{interval}")]
pub async fn get_traffic(interval: web::Path<String>) -> Result<HttpResponse> {
    if !check_interval_validate(interval.to_string()) {
        println!("");
        return Ok(HttpResponse::BadRequest()
            .json(json!(ResponseError::new_response("invalid interval", 404))));
    }

    match Database::default()?
        .connect()?
        .select_table::<Traffic>(interval.to_string())
    {
        Ok(result) => {
            Ok(HttpResponse::Ok().json(json!(Response::<Vec<Traffic>>::new("success", result))))
        }
        Err(err) => Ok(
            HttpResponse::BadRequest().json(json!(ResponseError::new_response(
                format!("{err:?}").as_str(),
                502
            ))),
        ),
    }
}

fn check_interval_validate(interval: String) -> bool {
    let available_intervals = Vec::from(["fiveminute", "hour", "day", "month", "year", "top"]);
    available_intervals.contains(&interval.as_str())
}
