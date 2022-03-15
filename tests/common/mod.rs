use once_cell::sync::OnceCell;
use rocket::{
    http::{ContentType, Cookie},
    local::blocking::{Client, LocalResponse},
};
use std::sync::Mutex;

pub const USERNAME: &'static str = "oddtest";
pub const EMAIL: &'static str = "oddtest@test.com";
pub const PASSWORD: &'static str = "password";

pub fn test_client() -> &'static Mutex<Client> {
    static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let rocket = rocket_rest_quickstart::rocket();
        Mutex::from(Client::tracked(rocket).expect("valid rocket instance"))
    })
}

// Auth utils
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

pub fn user_id_cookie(response: &LocalResponse<'_>) -> Option<Cookie<'static>> {
    let cookie = response
        .headers()
        .get("Set-Cookie")
        .filter(|v| v.starts_with("user_id"))
        .nth(0)
        .and_then(|val| Cookie::parse_encoded(val).ok());

    cookie.map(|c| c.into_owned())
}
