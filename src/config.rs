use rocket::config::Config;
use rocket::figment::{
    map,
    value::{Map, Value},
    Figment,
};
use std::env;

pub fn from_env() -> Figment {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // In a production environment this key is needed to encrypt private cookies
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => 10.into()
    };

    Config::figment()
        .merge(("databases", map!["diesel_postgres_pool" => db]))
        .merge(("secret_key", secret_key))
}
