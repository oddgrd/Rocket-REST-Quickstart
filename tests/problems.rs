mod common;
use common::*;
use rocket::http::Status;

#[test]
fn greeting() {
    let client = test_client();
    let response = client.get("/api/Odd").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello, Odd!".into()));
}
