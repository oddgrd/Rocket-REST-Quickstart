#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;
use rocket::{
    fairing::AdHoc,
    figment::{
        map,
        value::{Map, Value},
    },
    Build, Rocket,
};
use std::env;

mod models;
mod routes;
mod schema;

use rocket_sync_db_pools::database;

#[database("mhb")]
pub struct DbPool(diesel::pg::PgConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();

    let conn = DbPool::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run_with_output(c, &mut std::io::stdout()))
        .await
        .expect("diesel migrations");

    rocket
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn rocket() -> rocket::Rocket<Build> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => 10.into()
    };

    let figment = rocket::Config::figment().merge(("databases", map!["mhb" => db]));

    rocket::custom(figment)
        .attach(DbPool::fairing())
        .attach(AdHoc::on_ignite("Database migrations", run_migrations))
        .mount(
            "/api",
            routes![
                index,
                routes::problems::create_problem,
                routes::problems::get_problem,
                routes::problems::delete_problem,
            ],
        )
}
