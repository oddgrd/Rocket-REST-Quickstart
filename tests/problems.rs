mod common;
use chrono::{DateTime, Utc};
use common::{login, register, PASSWORD, TEST_CLIENT};
use rocket::http::{ContentType, Cookie, Status};
use rocket::local::blocking::{Client, LocalResponse};
use serde_json::Value;
const PROBLEM_GRADE: i32 = 5;

#[test]
fn create_problem_creates_and_persists_problem() {
    let client = TEST_CLIENT.lock().unwrap();
    let cookie = login(&client);

    // Returns a 422 for invalid input
    let title = "test title too long for validation";
    let response = create_problem(&client, &cookie, title);
    assert_eq!(response.status(), Status::UnprocessableEntity);

    let title = "test get_problem";
    let response = create_problem(&client, &cookie, title);

    assert_eq!(response.status(), Status::Created);
    assert!(response
        .headers()
        .get_one("Location")
        .unwrap()
        .starts_with("/api/problems/"));

    let new_problem: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();

    let response = client
        .get(format!("/api/problems/{}", new_problem["id"]))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let problem: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();

    assert_eq!(problem["title"], title);
    assert_eq!(problem["grade"], PROBLEM_GRADE);
}

#[test]
fn get_problems() {
    const N: usize = 3;
    let client = TEST_CLIENT.lock().unwrap();
    let cookie = login(&client);

    // Create N new problems
    for i in 1..=N {
        let title = format!("test_get_problems_{}", i);
        create_problem(&client, &cookie, &title);
    }

    let response = client.get("/api/problems").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let problems: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();

    // Problems should be ordered by created_at DESC (newest first)
    assert_eq!(problems[0]["title"], "test_get_problems_3");
    assert_eq!(problems[N - 1]["title"], "test_get_problems_1");
    assert!(problems.as_array().unwrap().len() >= N);
}

#[test]
fn update_problem() {
    let client = TEST_CLIENT.lock().unwrap();
    let cookie = login(&client);

    let title = "test update_problem";
    let response = create_problem(&client, &cookie, title);
    let problem: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();

    let response = client
        .put(format!("/api/problems/{}", problem["id"]))
        .cookie(cookie.clone())
        .header(ContentType::Form)
        .body(format!("title=updated_problem&grade=13"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let updated_problem: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();

    // Verify that updated_at gets set
    let created_at: DateTime<Utc> =
        serde_json::from_str(&problem["createdAt"].to_string()).unwrap();
    let updated_at: DateTime<Utc> =
        serde_json::from_str(&updated_problem["updatedAt"].to_string()).unwrap();
    assert!(updated_at > created_at);

    assert_eq!(updated_problem["title"], "updated_problem".to_string());
    assert_eq!(updated_problem["grade"], 13);
}

#[test]
fn delete_problem() {
    let client = TEST_CLIENT.lock().unwrap();
    let cookie = login(&client);

    let title = "test delete_problem";
    let response = create_problem(&client, &cookie, title);
    let problem: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();

    let response = client
        .delete(format!("/api/problems/{}", problem["id"]))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn cannot_delete_or_update_when_not_creator() {
    let client = TEST_CLIENT.lock().unwrap();
    let cookie = login(&client);

    let title = "test not creator";
    let response = create_problem(&client, &cookie, title);
    let problem: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();

    // Register and get cookie of a different user
    let cookie =
        register(&client, "diff_user", "diff_user@test.com", PASSWORD).expect("cookie set");
    assert!(!cookie.value().is_empty());

    // User shouldn't be able to update other users problems
    let response = client
        .put(format!("/api/problems/{}", problem["id"]))
        .cookie(cookie.clone())
        .header(ContentType::Form)
        .body(format!("title=try_update&grade=13"))
        .dispatch();
    assert_eq!(response.status(), Status::InternalServerError);

    // User shouldn't be able to delete other users problems
    let response = client
        .delete(format!("/api/problems/{}", problem["id"]))
        .cookie(cookie.clone())
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

// Utils
fn create_problem<'a>(client: &'a Client, cookie: &Cookie, title: &str) -> LocalResponse<'a> {
    let response = client
        .post("/api/problems")
        .cookie(cookie.clone())
        .header(ContentType::Form)
        .body(format!("title={}&grade={}", title, PROBLEM_GRADE))
        .dispatch();

    response
}
