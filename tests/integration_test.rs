extern crate coloroo_api;
extern crate rocket;
extern crate rocket_contrib;

use rocket::http::Status;
use rocket::local::Client;

fn get_client() -> Client {
    Client::new(coloroo_api::rocket()).expect("valid rocket client")
}

#[test]
fn root_get_must_return_200_helloworld() {
    let client = get_client();
    let mut res = client.get("/").dispatch();

    assert_eq!(res.status(), Status::Ok);

    let body = res.body_string().unwrap();
    assert!(body.contains("Hello World!"));
}

#[test]
fn when_db_up_then_get_health_should_return_200_ok() {
    let client = get_client();
    let res = client.get("/health").dispatch();

    assert_eq!(res.status(), Status::new(200, "OK"));
}

#[test]
fn when_db_down_then_get_health_should_return_503() {
    // -- we cannot test this right now because the launch aborts when the db is down
}

#[test]
fn get_metadata_must_return_200() {
    let client = get_client();
    let res = client.get("/metadata").dispatch();

    assert_eq!(res.status(), Status::Ok);
}
