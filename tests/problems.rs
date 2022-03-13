mod common;
use common::{login, test_client, Problem};
use rocket::http::{ContentType, Cookie, Status};
use rocket::local::blocking::Client;

const PROBLEM_GRADE: i32 = 5;

#[test]
fn post_problem() {
    let client = test_client().lock().unwrap();
    let cookie = login(&client);
    let title = "test post_problem";

    let problem: Problem = create_problem(&client, &cookie, title);

    assert_eq!(problem.title, title);
    assert_eq!(problem.grade, PROBLEM_GRADE);
}

#[test]
fn get_problem() {
    let client = test_client().lock().unwrap();
    let cookie = login(&client);

    let title = "test get_problem";
    let new_problem: Problem = create_problem(&client, &cookie, title);

    let response = client
        .get(format!("/api/problems/{}", new_problem.id))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let problem: Problem = response.into_json().unwrap();

    assert_eq!(problem.title, title);
    assert_eq!(problem.grade, PROBLEM_GRADE);
}

#[test]
fn get_problems() {
    const N: usize = 3;
    let client = test_client().lock().unwrap();
    let cookie = login(&client);

    // Create N new problems
    for i in 1..=N {
        let title = format!("test_get_problems_{}", i);
        create_problem(&client, &cookie, &title);
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
    let cookie = login(&client);

    let title = "test update_problem";
    let new_problem: Problem = create_problem(&client, &cookie, title);

    let response = client
        .put(format!("/api/problems/{}", new_problem.id))
        .cookie(cookie.clone())
        .header(ContentType::Form)
        .body(format!("title=updated_problem&grade=13"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let updated_problem: Problem = response.into_json().unwrap();

    assert!(updated_problem.updated_at > new_problem.created_at);
    assert_eq!(updated_problem.title, "updated_problem".to_string());
    assert_eq!(updated_problem.grade, 13);
}

#[test]
fn delete_problem() {
    let client = test_client().lock().unwrap();
    let cookie = login(&client);

    let title = "test delete_problem";
    let problem: Problem = create_problem(&client, &cookie, title);

    let response = client
        .delete(format!("/api/problems/{}", problem.id))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

// Utils
fn create_problem(client: &Client, cookie: &Cookie, title: &str) -> Problem {
    let response = client
        .post("/api/problems")
        .cookie(cookie.clone())
        .header(ContentType::Form)
        .body(format!("title={}&grade={}", title, PROBLEM_GRADE))
        .dispatch();

    assert_eq!(response.status(), Status::Created);
    assert!(response
        .headers()
        .get_one("Location")
        .unwrap()
        .starts_with("/api/problems/"));

    response.into_json().unwrap()
}
