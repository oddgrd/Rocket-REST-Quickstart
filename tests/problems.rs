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
    let title = "test get_problem";
    let problem: Problem = create_problem(&client, title);

    let response: Problem = client
        .get(format!("/api/problems/{}", problem.id))
        .dispatch()
        .into_json()
        .unwrap();

    assert_eq!(response.grade, PROBLEM_GRADE);
    assert_eq!(response.rating, PROBLEM_RATING);
    assert_eq!(response.title, title);
}

#[test]
fn get_problems() {
    const N: usize = 10;
    let client = test_client();

    // Create N new problems
    for i in 1..=N {
        let title = format!("test_get_problems_{}", i);
        create_problem(&client, &title);
    }

    let response: Vec<Problem> = client.get("/api/problems").dispatch().into_json().unwrap();

    // Problems are ordered by created_at DESC
    assert_eq!(response[0].title, "test_get_problems_10");
    assert_eq!(response[9].title, "test_get_problems_1");
    assert!(response.len() >= 10);
}

#[test]
fn update_problem() {
    let client = test_client();
    let title = "test update_problem";
    let problem: Problem = create_problem(&client, title);

    let response: Problem = client
        .put(format!("/api/problems/{}", problem.id))
        .header(ContentType::JSON)
        .body(json_string!({
            "title": "updated_problem".to_string(),
            "grade": 13,
            "rating": 2,
        }))
        .dispatch()
        .into_json()
        .unwrap();

    assert_eq!(response.title, "updated_problem".to_string());
    assert_eq!(response.grade, 13);
    assert_eq!(response.rating, 2);
}

#[test]
fn delete_problem() {
    let client = test_client();
    let title = "test delete_problem";
    let problem: Problem = create_problem(&client, title);

    let response = client
        .delete(format!("/api/problems/{}", problem.id))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

// Utils
fn create_problem(client: &Client, title: &str) -> Problem {
    let response = client
        .post("/api/problems")
        .header(ContentType::JSON)
        .body(json_string!({
            "title": title,
            "grade": PROBLEM_GRADE,
            "rating": PROBLEM_RATING
        }))
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    response.into_json().unwrap()
}
