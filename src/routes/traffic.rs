use rocket::*;
use rocket_contrib;
use serde::Serialize;

#[get("/traffic/<interval>")]
pub fn get_traffic(interval: String) -> Result<(), String> {
    Ok(())
}
