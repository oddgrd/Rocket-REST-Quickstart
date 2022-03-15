mod common;
use common::{login, register, test_client};
use rocket::http::{ContentType, Cookie, Status};
use rocket::local::blocking::{Client, LocalResponse};
use rocket_rest_quickstart::models::problem::Problem;
const PROBLEM_GRADE: i32 = 5;

#[test]
fn create_and_get_problem() {
    let client = test_client().lock().unwrap();
    let cookie = login(&client);

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

    let new_problem: Problem = response.into_json().unwrap();

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
    let new_problem: Problem = create_problem(&client, &cookie, title).into_json().unwrap();

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
    let problem: Problem = create_problem(&client, &cookie, title).into_json().unwrap();

    let response = client
        .delete(format!("/api/problems/{}", problem.id))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn delete_update_different_user() {
    let client = test_client().lock().unwrap();
    let cookie = login(&client);

    let title = "test not creator";
    let problem: Problem = create_problem(&client, &cookie, title).into_json().unwrap();

    // Register and get cookie of a different user
    let cookie =
        register(&client, "diff_user", "diff_user@test.com", "password").expect("cookie set");
    assert!(!cookie.value().is_empty());

    // User shouldn't be able to update other users problems
    let response = client
        .put(format!("/api/problems/{}", problem.id))
        .cookie(cookie.clone())
        .header(ContentType::Form)
        .body(format!("title=try_update&grade=13"))
        .dispatch();
    assert_eq!(response.status(), Status::InternalServerError);

    // User should n'tbe able to delete other users problems
    let response = client
        .delete(format!("/api/problems/{}", problem.id))
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
