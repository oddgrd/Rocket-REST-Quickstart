mod common;
use common::{test_client, Profile, User};
use rocket::{
    http::{ContentType, Cookie, Status},
    local::blocking::{Client, LocalResponse},
};

#[test]
fn register_new_user() {
    let client = test_client().lock().unwrap();

    let username = "odd";
    let email = "odd@test.com";
    let password = "password";

    let new_user = register(&client, username, email, password);

    assert_eq!(new_user.username, username);
    assert_eq!(new_user.email, email);
}

#[test]
fn login_user() {
    let client = test_client().lock().unwrap();

    let username = "oddLogin";
    let email = "oddLogin@test.com";
    let password = "password";

    let new_user = register(&client, username, email, password);

    let login_cookie = login(&client, username, password).expect("logged in");

    // Verify logged in
    let response = client
        .get("/api/users/me")
        .cookie(login_cookie.clone())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let db_user: User = response.into_json().unwrap();

    assert_eq!(db_user.id, new_user.id);
    assert_eq!(db_user.username, username);
}

#[test]
fn login_incorrect_input() {
    let client = test_client().lock().unwrap();

    let username = "oddLoginErr";
    let email = "oddLoginErr@test.com";
    let password = "password";

    register(&client, username, email, password);

    assert!(login(&client, username, "wrong").is_none());
    assert!(login(&client, "wrong", password).is_none());
}

#[test]
fn get_profile_by_id() {
    let client = test_client().lock().unwrap();
    let username = "oddGet";
    let email = "oddget@test.com";

    let new_user = register(&client, username, email, "password");

    let response = client.get(format!("/api/users/{}", new_user.id)).dispatch();

    assert_eq!(response.status(), Status::Ok);

    let user: Profile = response.into_json().unwrap();

    assert_eq!(user.username, username);
}

// Utils
fn register(client: &Client, username: &str, email: &str, password: &str) -> User {
    let response = client
        .post("/api/users/register")
        .header(ContentType::Form)
        .body(format!(
            "username={}&email={}&password={}",
            username, email, password
        ))
        .dispatch();

    assert_eq!(response.status(), Status::Created);
    assert!(user_id_cookie(&response).is_some());

    response.into_json().unwrap()
}

fn login(client: &Client, username: &str, password: &str) -> Option<Cookie<'static>> {
    let response = client
        .post("/api/users/login")
        .header(ContentType::Form)
        .body(format!("username={}&password={}", username, password))
        .dispatch();

    user_id_cookie(&response)
}

fn user_id_cookie(response: &LocalResponse<'_>) -> Option<Cookie<'static>> {
    let cookie = response
        .headers()
        .get("Set-Cookie")
        .filter(|v| v.starts_with("user_id"))
        .nth(0)
        .and_then(|val| Cookie::parse_encoded(val).ok());

    cookie.map(|c| c.into_owned())
}
