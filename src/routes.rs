use db::ColorsDbCon;
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
pub fn get_health(db_con: ColorsDbCon) -> Status {
    match rocket_contrib::databases::redis::cmd("PING").query::<()>(&*db_con) {
        Ok(_) => rocket::http::Status::Ok,
        Err(_) => Status::new(503, "PING FAIL"),
    }
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
