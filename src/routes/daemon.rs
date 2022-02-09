use rocket::*;
use rocket_contrib::json::Json;
use serde::Serialize;
#[get("/daemon")]
pub fn get_daemon_status() -> Result<(), String> {
    Ok(())
}

#[post("/daemon/<status>")]
pub fn change_daemon_status(status: String) -> Result<(), String> {
    Ok(())
}
