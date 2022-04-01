#![allow(unused)]
use once_cell::sync::OnceCell;
use rocket::{
    http::{ContentType, Cookie},
    local::blocking::{Client, LocalResponse},
};
use std::sync::Mutex;

pub const USERNAME: &'static str = "oddtest";
pub const EMAIL: &'static str = "oddtest@test.com";
pub const PASSWORD: &'static str = "passwordtest";

/// Launch test_client in a OnceCell to share memory between threads,
/// as well as making sure it is only initialized once. The data inside
/// is protected by a Mutex, only one test can hold the lock at a time
/// and write to the DB
///
/// ## Implementation notes
/// This strategy avoids race conditions, but at the cost of test speed,
/// as only one test can hold the mutex lock at a time. The alternative is to
/// create a new DB for each test, or use test transactions to rollback queries.
pub fn test_client() -> &'static Mutex<Client> {
    static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let rocket = rocket_rest_quickstart::rocket();
        Mutex::from(Client::tracked(rocket).expect("valid rocket instance"))
    })
}

/// Attempt login, fall back to register and then retry login
pub fn login(client: &Client) -> Cookie<'static> {
    try_login(client).unwrap_or_else(|| {
        register(client, USERNAME, EMAIL, PASSWORD);
        try_login(client).expect("Cannot login")
    })
}

pub fn try_login(client: &Client) -> Option<Cookie<'static>> {
    let response = client
        .post("/api/users/login")
        .header(ContentType::Form)
        .body(format!("username={}&password={}", USERNAME, PASSWORD))
        .dispatch();

    user_id_cookie(&response)
}

/// Register new user, sign in and return cookie
pub fn register(
    client: &Client,
    username: &str,
    email: &str,
    password: &str,
) -> Option<Cookie<'static>> {
    let response = client
        .post("/api/users/register")
        .header(ContentType::Form)
        .body(format!(
            "username={}&email={}&password={}",
            username, email, password
        ))
        .dispatch();

    user_id_cookie(&response)
}

/// Return cookie from response headers if it exists
pub fn user_id_cookie(response: &LocalResponse<'_>) -> Option<Cookie<'static>> {
    let cookie = response
        .headers()
        .get("Set-Cookie")
        .filter(|v| v.starts_with("user_id"))
        .nth(0)
        .and_then(|val| Cookie::parse_encoded(val).ok());

    cookie.map(|c| c.into_owned())
}
