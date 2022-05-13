mod common;
use common::{login, test_client, user_id_cookie, EMAIL, PASSWORD, USERNAME};
use rocket::http::{ContentType, Status};
use serde_json::Value;

#[test]
fn register_returns_a_422_for_invalid_input() {
    let client = test_client().lock().unwrap();

    let test_cases = [
        (USERNAME, "invalid_email.com", PASSWORD),
        ("a", "test@gmail.com", PASSWORD),
        ("validname", "test@gmail.com", "too_short"),
    ];

    for (username, email, password) in test_cases {
        let response = client
            .post("/api/users/register")
            .header(ContentType::Form)
            .body(format!(
                "username={}&email={}&password={}",
                username, email, password
            ))
            .dispatch();

        assert_eq!(response.status(), Status::UnprocessableEntity)
    }
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

    let db_user: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();

    assert_eq!(db_user["email"], EMAIL);
    assert_eq!(db_user["username"], USERNAME);
}

#[test]
fn login_incorrect_input() {
    let client = test_client().lock().unwrap();

    // Make sure a user is created with default values
    let _user = login(&client);

    let test_cases = [
        ("nonexistant user", PASSWORD, "user doesn't exist"),
        (USERNAME, "wrong password", "invalid password"),
    ];

    for (username, password, error_message) in test_cases {
        let response = client
            .post("/api/users/login")
            .header(ContentType::Form)
            .body(format!("username={}&password={}", username, password))
            .dispatch();

        assert_eq!(response.status(), Status::Unauthorized);
        assert!(response.into_string().unwrap().contains(error_message));
    }
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

    let user: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();

    // Get profile
    let response = client.get(format!("/api/users/{}", user["id"])).dispatch();
    assert_eq!(response.status(), Status::Ok);

    let profile: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert_eq!(profile["username"], USERNAME);
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
}
