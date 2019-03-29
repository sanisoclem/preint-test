extern crate rocket;
extern crate rocket_contrib;
extern crate rustaroo_api;

use rocket::http::Status;
use rocket::local::Client;

fn get_client() -> Client {
    Client::new(rustaroo_api::rocket()).expect("valid rocket client")
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
fn health_should_return_204_no_content() {
    let client = get_client();
    let res = client.get("/health").dispatch();

    assert_eq!(res.status(), Status::NoContent);
}

#[test]
fn get_metadata_must_return_200() {
    let client = get_client();
    let res = client.get("/metadata").dispatch();

    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn nonexistent_route_returns_404() {
    let client = get_client();
    let res = client.get("/invalid").dispatch();

    assert_eq!(res.status(), Status::NotFound);
}
