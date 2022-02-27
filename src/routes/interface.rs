use rocket::*;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::io::{
    Error,
    ErrorKind::{Interrupted, InvalidData, InvalidInput, NotFound},
};

use crate::db::{models::interface::Interface, Database};

#[get("/interface")]
pub fn get_interfaces() -> Result<Json<Vec<Interface>>, Json<Error>> {
    match Database::default()
        .unwrap()
        .connect()
        .unwrap()
        .select_table::<Interface>("interface".to_owned())
    {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Json(Error::new(Interrupted, format!("{}", err)))),
    }
}
