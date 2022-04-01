mod common;
use common::test_client;
use rocket::http::Status;

#[test]
fn health_check_works() {
    let client = test_client().lock().unwrap();

    let response = client.get("/api/health_check").dispatch();

    assert_eq!(response.status(), Status::Ok);
}
