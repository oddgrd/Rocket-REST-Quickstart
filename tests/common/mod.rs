use chrono::{DateTime, Utc};
use once_cell::sync::OnceCell;
use rocket::local::blocking::Client;
use serde::Deserialize;
use std::sync::Mutex;

#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
    };
}

pub fn test_client() -> &'static Mutex<Client> {
    static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let rocket = rocket_pg_template::rocket();
        Mutex::from(Client::tracked(rocket).expect("valid rocket instance"))
    })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    pub id: i32,
    pub title: String,
    pub grade: i32,
    pub rating: i32,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct Profile {
    pub id: i32,
    pub username: String,
    pub created_at: DateTime<Utc>,
}
