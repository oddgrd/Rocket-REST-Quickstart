use rocket_rest_quickstart::configuration;
#[rocket::main]
async fn main() {
    if let Err(e) = rocket_rest_quickstart::startup::rocket(configuration::get_configuration())
        .launch()
        .await
    {
        println!("Rocket didn't launch");
        drop(e);
    };
}
