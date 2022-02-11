#[rocket::main]
async fn main() {
    if let Err(e) = rocket_pg_template::rocket().launch().await {
        println!("Rocket didn't launch");
        drop(e);
    };
}
