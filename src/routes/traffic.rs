use std::vec;

use rocket::*;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::db::{models::traffic::Traffic, Database};
use std::io::{
    Error,
    ErrorKind::{Interrupted, InvalidData, InvalidInput, NotFound},
};

#[get("/traffic/<interval>")]
pub fn get_traffic(interval: String) -> Result<Json<Vec<Traffic>>, Json<Error>> {
    let available_intervals = Vec::from(["fiveminute", "hour", "day", "month", "year", "top"]);

    if !available_intervals.contains(&interval.as_str()) {
        return Err(Json(Error::new(InvalidInput, "invalid interval")));
    };
    match Database::default()
        .unwrap()
        .connect()
        .unwrap()
        .select_table::<Traffic>(interval)
    {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Json(Error::new(Interrupted, format!("{}", err)))),
    }
}
