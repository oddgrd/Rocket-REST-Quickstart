use rocket::{
    config::Config,
    figment::{
        map,
        value::{Map, Value},
        Figment,
    },
};
use std::env;

pub fn from_env() -> Figment {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => 10.into()
    };

    Config::figment().merge(("databases", map!["diesel_postgres_pool" => db]))
}
