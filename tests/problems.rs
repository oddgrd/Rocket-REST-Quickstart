mod common;
use chrono::{DateTime, Utc};
use common::*;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use serde::Deserialize;

const PROBLEM_GRADE: i32 = 5;
const PROBLEM_RATING: i32 = 1;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    pub id: i32,
    pub title: String,
    pub grade: i32,
    pub rating: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[test]
fn post_problem() {
    let client = test_client();
    let test_title = "test post_problem";

    let problem: Problem = create_problem(&client, test_title);

    assert_eq!(problem.grade, PROBLEM_GRADE);
    assert_eq!(problem.rating, PROBLEM_RATING);
    assert_eq!(problem.title, test_title);
}

#[test]
fn get_problem() {
    let client = test_client();
    let test_title = "test get_problem";
    let problem: Problem = create_problem(&client, test_title);

    let response: Problem = client
        .get(format!("/api/problems/{}", problem.id))
        .dispatch()
        .into_json()
        .unwrap();

    assert_eq!(response.grade, PROBLEM_GRADE);
    assert_eq!(response.rating, PROBLEM_RATING);
    assert_eq!(response.title, test_title);
}

#[test]
fn delete_problem() {
    let client = test_client();
    let test_title = "test delete_problem";
    let problem: Problem = create_problem(&client, test_title);

    let response = client
        .delete(format!("/api/problems/{}", problem.id))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

// Utils
fn create_problem<'a>(client: &'a Client, test_title: &str) -> Problem {
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

    response.into_json().unwrap()
}
