mod common;
use chrono::{DateTime, Utc};
use common::{login, test_client, user_id_cookie, EMAIL, PASSWORD, USERNAME};
use rocket::http::{ContentType, Status};
use rocket_rest_quickstart::models::user::Profile;
use serde::Deserialize;

// User model for tests since lib model skip_serialize bugs tests
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[test]
fn login_or_register() {
    let client = test_client().lock().unwrap();
    let new_user_cookie = login(&client);

    // Verify that user is persisted in DB
    let response = client
        .get("/api/users/me")
        .cookie(new_user_cookie.clone())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let db_user: User = response.into_json().unwrap();

    assert_eq!(db_user.email, EMAIL);
    assert_eq!(db_user.username, USERNAME);
}

#[test]
fn login_incorrect_input() {
    let client = test_client().lock().unwrap();

    // Make sure a user is created
    let _user = login(&client);

    // Incorrect username
    let response = client
        .post("/api/users/login")
        .header(ContentType::Form)
        .body(format!("username={}&password={}", "wrong", PASSWORD))
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    assert!(response
        .into_string()
        .unwrap()
        .contains("user doesn't exist"));

    // Incorrect password
    let response = client
        .post("/api/users/login")
        .header(ContentType::Form)
        .body(format!("username={}&password={}", USERNAME, "wrong"))
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    assert!(response.into_string().unwrap().contains("invalid password"));
}

#[test]
fn get_profile_by_id() {
    let client = test_client().lock().unwrap();
    let login_cookie = login(&client);

    // Get user from DB
    let response = client
        .get("/api/users/me")
        .cookie(login_cookie.clone())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let user: User = response.into_json().unwrap();

    // Get profile
    let response = client.get(format!("/api/users/{}", user.id)).dispatch();
    assert_eq!(response.status(), Status::Ok);

    let profile: Profile = response.into_json().unwrap();
    assert_eq!(profile.username, USERNAME);
}

#[test]
fn logout() {
    let client = test_client().lock().unwrap();
    let login_cookie = login(&client);

    let response = client
        .post("/api/users/logout")
        .cookie(login_cookie)
        .dispatch();

    let cookie = user_id_cookie(&response).expect("logout cookie");
    assert!(cookie.value().is_empty());

    // User should be redirected
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location").unwrap(), "/api");

    // Page should show success message
    let response = client.get("/api").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().unwrap();
    assert!(body.contains("Successfully logged out."));
}
