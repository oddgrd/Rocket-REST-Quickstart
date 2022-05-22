use dotenv::dotenv;
use rocket::config::Config;
use rocket::figment::{
    map,
    value::{Map, Value},
    Figment,
};
use std::env;
pub fn get_configuration() -> Figment {
    dotenv().ok();
    let database_base_url = env::var("DATABASE_BASE_URL").expect("DATABASE_BASE_URL must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let database_url = format!("{}/{}", database_base_url, database_name);

    // In a production environment this key is needed to encrypt private cookies
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let db: Map<_, Value> = map! {
        "url" => database_url.into(),
        "pool_size" => 10.into()
    };

    Config::figment()
        .merge(("databases", map!["diesel_postgres_pool" => db]))
        .merge(("secret_key", secret_key))
}
