use rocket::local::blocking::Client;

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
