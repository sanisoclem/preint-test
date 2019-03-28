#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod db;
mod meta;
mod routes;

pub fn rocket() -> rocket::Rocket {
    // TODO: set cors headers
    // TODO: setup routes in separate module
    rocket::ignite()
        .mount(
            "/",
            routes![routes::get, routes::get_metadata, routes::get_health],
        )
        .register(catchers![routes::not_found, routes::internal_error])
        .attach(db::ColorsDbCon::fairing())
}