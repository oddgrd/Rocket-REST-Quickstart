use diesel::{Connection, PgConnection};
use once_cell::sync::Lazy;
use rocket::{
    http::{ContentType, Cookie},
    local::blocking::{Client, LocalResponse},
};
use rocket_rest_quickstart::config::{Configuration, DatabaseSettings};
use std::sync::Mutex;

pub const USERNAME: &'static str = "oddtest";
pub const EMAIL: &'static str = "oddtest@test.com";
pub const PASSWORD: &'static str = "passwordtest";

// ## Implementation notes
// This strategy avoids race conditions, but at the cost of test speed.
// As only one test can hold the mutex lock at a time, the tests run synchronously.
// The rocket official examples also use a mutex for testing.
// A better solution for testing against a database is likely to be implemented
// in the future.
//
/// Ensures that the `TEST_CLIENT` is only initialised once using `once_cell`.
/// The data inside is protected by a Mutex, only one test can hold the lock at
/// a time and write to the DB, preventing conflicts.
/// ### Example:
/// ```
/// // Acquire lock
/// TEST_CLIENT.lock().unwrap()
/// // Act
/// let response = client.get("/api/health_check").dispatch();
/// // Assert
/// assert_eq!(response.status(), Status::Ok);
/// ```
pub static TEST_CLIENT: Lazy<Mutex<Client>> = Lazy::new(|| {
    // Get configuration with random database name
    let configuration = Configuration::from_env().with_test_db();

    configure_test_database(&configuration.database_settings);

    let rocket = rocket_rest_quickstart::startup::rocket(configuration.build());
    Mutex::from(Client::tracked(rocket).expect("valid rocket instance"))
});

/// Setup a new DB each time the tests run
fn configure_test_database(settings: &DatabaseSettings) {
    // Connect to postgres
    let connection = PgConnection::establish(&format!("{}/postgres", settings.base_url))
        .expect("Failed to connect to Postgres");

    // Create new database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, settings.name).as_str())
        .expect("Failed to create database.");
}

/// Attempt login, fall back to register and then retry login
pub fn login(client: &Client) -> Cookie<'static> {
    try_login(client).unwrap_or_else(|| {
        register(client, USERNAME, EMAIL, PASSWORD);
        try_login(client).expect("Cannot login")
    })
}

/// Attempt login
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
