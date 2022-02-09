use rocket::*;
use rocket_contrib::json::Json;
use serde::Serialize;

#[get("/config")]
pub fn get_configs() -> Result<(), String> {
    Ok(())
}

#[post("/config")]
pub fn edit_configs() -> Result<(), String> {
    Ok(())
}
