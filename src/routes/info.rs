use rocket::*;
use rocket_contrib::json::Json;
use serde::Serialize;

#[get("/info")]
pub fn get_info() -> Result<(), String> {
    Ok(())
}
