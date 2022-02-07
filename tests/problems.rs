mod common;
use common::*;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::{Client, LocalResponse};

const PROBLEM_GRADE: i32 = 5;
const PROBLEM_RATING: i32 = 1;

#[test]
fn post_problem() {
    let client = test_client();
    let test_title = "test post_problem";
    let response = create_problem(&client, test_title);

    let value = response_json_value(response);

    assert_eq!(value["grade"], PROBLEM_GRADE);
    assert_eq!(value["rating"], PROBLEM_RATING);
    assert_eq!(value["title"], test_title);
}

#[test]
fn get_problem() {
    let client = test_client();
    let test_title = "test get_problem";
    let problem = response_json_value(create_problem(&client, test_title));

    let response = client
        .get(format!("/api/problems/{}", problem["id"]))
        .dispatch();
    let value = response_json_value(response);

    assert_eq!(value["grade"], PROBLEM_GRADE);
    assert_eq!(value["rating"], PROBLEM_RATING);
    assert_eq!(value["title"], test_title);
}

// Utils
fn create_problem<'a>(client: &'a Client, test_title: &str) -> LocalResponse<'a> {
    let response = client
        .post("/api/problems")
        .header(ContentType::JSON)
        .body(json_string!({
            "title": test_title,
            "grade": PROBLEM_GRADE,
            "rating": PROBLEM_RATING
        }))
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    response
}
