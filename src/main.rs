#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};

// Instace of app metadata (compile-time) for the current app
pub const APP_META: AppMetadata<'static> = AppMetadata {
    name: env!("CARGO_PKG_NAME"),
    description: env!("CARGO_PKG_DESCRIPTION"),
    version: env!("CARGO_PKG_VERSION"),
    sha: option_env!("RUSTAROO_SHA"),
};

// Metadata describing an application
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMetadata<'a> {
    name: &'a str,
    description: &'a str,
    version: &'a str,
    sha: Option<&'a str>,
}

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
pub fn get_health() -> JsonValue {
    json!({
      "status": "Healthy"
    })
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

fn rocket() -> rocket::Rocket {
    // TODO: set cors headers
    // TODO: attach db
    // TODO: setup routes in separate module
    rocket::ignite()
        .mount("/", routes![get, get_metadata, get_health])
        .register(catchers![not_found, internal_error])
}

fn main() {
    rocket().launch();
}
