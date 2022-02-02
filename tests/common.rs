use rocket::local::blocking::Client;
use rocket_pg_template;

pub fn test_client() -> Client {
    let rocket = rocket_pg_template::rocket();
    Client::tracked(rocket).expect("valid rocket instance")
}
