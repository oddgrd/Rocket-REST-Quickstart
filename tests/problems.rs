mod common;
use common::*;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

const PROBLEM_GRADE: i32 = 5;
const PROBLEM_RATING: i32 = 1;

#[test]
fn post_problem() {
    let client = test_client().lock().unwrap();
    let test_title = "test post_problem";

    let problem: Problem = create_problem(&client, test_title);

    assert_eq!(problem.title, test_title);
    assert_eq!(problem.grade, PROBLEM_GRADE);
    assert_eq!(problem.rating, PROBLEM_RATING);
}

#[test]
fn get_problem() {
    let client = test_client().lock().unwrap();
    let title = "test get_problem";
    let new_problem: Problem = create_problem(&client, title);

    let response = client
        .get(format!("/api/problems/{}", new_problem.id))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let problem: Problem = response.into_json().unwrap();

    assert_eq!(problem.title, title);
    assert_eq!(problem.grade, PROBLEM_GRADE);
    assert_eq!(problem.rating, PROBLEM_RATING);
}

#[test]
fn get_problems() {
    const N: usize = 3;
    let client = test_client().lock().unwrap();

    // Create N new problems
    for i in 1..=N {
        let title = format!("test_get_problems_{}", i);
        create_problem(&client, &title);
    }

    let response = client.get("/api/problems").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let problems: Vec<Problem> = response.into_json().unwrap();

    // Problems should be ordered by created_at DESC (newest first)
    assert_eq!(problems[0].title, "test_get_problems_3");
    assert_eq!(problems[N - 1].title, "test_get_problems_1");
    assert!(problems.len() >= N);
}

#[test]
fn update_problem() {
    let client = test_client().lock().unwrap();
    let title = "test update_problem";
    let new_problem: Problem = create_problem(&client, title);

    let response = client
        .put(format!("/api/problems/{}", new_problem.id))
        .header(ContentType::JSON)
        .body(json_string!({
            "title": "updated_problem".to_string(),
            "grade": 13,
            "rating": 2,
        }))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let problem: Problem = response.into_json().unwrap();

    assert!(problem.updated_at > new_problem.created_at);
    assert_eq!(problem.title, "updated_problem".to_string());
    assert_eq!(problem.grade, 13);
    assert_eq!(problem.rating, 2);
}

#[test]
fn delete_problem() {
    let client = test_client().lock().unwrap();
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
    assert!(response
        .headers()
        .get_one("Location")
        .unwrap()
        .starts_with("/api/problems/"));

    response.into_json().unwrap()
}
