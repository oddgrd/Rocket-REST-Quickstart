mod common;
use common::*;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::{Client, LocalResponse};

const PROBLEM_TITLE: &str = "Test Problem";
const PROBLEM_GRADE: i32 = 5;
const PROBLEM_RATING: i32 = 1;

#[test]
fn greeting() {
    let client = test_client();
    let response = client.get("/api/Odd").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello, Odd!".into()));
}

#[test]
fn post_problem() {
    let client = test_client();
    let response = create_problem(&client);

    let value = response_json_value(response);

    assert_eq!(value["grade"], PROBLEM_GRADE);
    assert_eq!(value["rating"], PROBLEM_RATING);
    assert_eq!(value["title"], PROBLEM_TITLE);
}

// Utils
fn create_problem(client: &Client) -> LocalResponse {
    let response = client
        .post("/api/problems")
        .header(ContentType::JSON)
        .body(json_string!({
            "title": PROBLEM_TITLE,
            "grade": PROBLEM_GRADE,
            "rating": PROBLEM_RATING
        }))
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    response
}
