use meta::{AppMetadata, APP_META};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

// root endpoint routes
#[get("/")]
pub fn get() -> &'static str {
    "Hello World!"
}

#[get("/metadata")]
pub fn get_metadata() -> Json<AppMetadata<'static>> {
    Json(APP_META)
}

#[get("/health")]
pub fn get_health() -> Status {
    // TODO check dependencies
    // TODO return 503 if unhealthy
    // TODO return info if unhealthy/degraded
    Status::NoContent
}

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(500)]
pub fn internal_error() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Server error."
    })
}
