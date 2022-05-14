mod common;
use common::TEST_CLIENT;
use rocket::http::Status;

#[test]
fn health_check_works() {
    let client = TEST_CLIENT.lock().unwrap();

    let response = client.get("/api/health_check").dispatch();

    assert_eq!(response.status(), Status::Ok);
}
