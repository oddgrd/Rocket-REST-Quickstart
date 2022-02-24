use rocket::local::blocking::Client;
use std::sync::Mutex;
use once_cell::sync::OnceCell;

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
