#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use rocket::{
    figment::{
        map,
        value::{Map, Value},
    },
    response::Debug,
    Build,
};
use std::env;

mod models;
mod routes;
mod schema;

use rocket_sync_db_pools::database;

#[database("mhb")]
pub struct DBPool(diesel::pg::PgConnection);

pub type QueryResult<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

pub fn rocket() -> rocket::Rocket<Build> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABUSE_URL must be set");
    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => 10.into()
    };

    let figment = rocket::Config::figment().merge(("databases", map!["mhb" => db]));
    rocket::custom(figment)
        .mount(
            "/api",
            routes![
                routes::problems::create_problem,
                routes::problems::get_problem
            ],
        )
        .attach(DBPool::fairing())
}
