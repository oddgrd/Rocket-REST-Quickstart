use rocket::local::blocking::{Client, LocalResponse};
use rocket::serde::json::Value;
use rocket_pg_template;

#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
    };
}

pub fn test_client() -> Client {
    let rocket = rocket_pg_template::rocket();
    Client::tracked(rocket).expect("valid rocket instance")
}

pub fn response_json_value(response: LocalResponse) -> Value {
    response.into_json().expect("no body")
}
