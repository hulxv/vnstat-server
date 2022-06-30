use crate::http::response::{Response, ResponseError, ResponseStatus};
use actix_web::{get, web, HttpResponse};
use libvnstat::VnStat;

#[get("/traffic/{interval}")]
pub async fn get_traffic(interval: web::Path<String>) -> HttpResponse {
    match VnStat.traffic(interval.as_str()).get() {
        Ok(result) => HttpResponse::Ok().json(
            Response::new()
                .status(ResponseStatus::Success)
                .data(&result)
                .build(),
        ),
        Err(err) => {
            if let Some(err) = err.root_cause().downcast_ref::<std::io::Error>() {
                if err.kind() == std::io::ErrorKind::InvalidInput {
                    return HttpResponse::NotFound().json(
                        ResponseError::new()
                            .code(404)
                            .details("Interval isn't found.")
                            .build(),
                    );
                }
            }
            HttpResponse::BadRequest().json(ResponseError::new().build())
        }
    }
}
