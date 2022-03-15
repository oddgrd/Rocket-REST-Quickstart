#[rocket::main]
async fn main() {
    if let Err(e) = rocket_rest_quickstart::rocket().launch().await {
        println!("Rocket didn't launch");
        drop(e);
    };
}
