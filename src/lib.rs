#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

pub mod meta;
mod routes;

pub fn rocket() -> rocket::Rocket {
    // TODO: set cors headers
    rocket::ignite()
        .mount(
            "/",
            routes![routes::get, routes::get_metadata, routes::get_health],
        )
        .register(catchers![routes::not_found, routes::internal_error])
}
