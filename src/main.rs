use rocket_rest_quickstart::config;

#[rocket::main]
async fn main() {
    if let Err(e) =
        rocket_rest_quickstart::startup::rocket(config::Configuration::from_env().build())
            .launch()
            .await
    {
        println!("Rocket didn't launch");
        drop(e);
    };
}
