#[get("/<name>")]
pub fn greeting(name: String) -> String {
    format!("Hello, {}!", name)
}
