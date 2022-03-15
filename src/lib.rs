#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate rocket_sync_db_pools;

use dotenv::dotenv;
use rocket::{fairing::AdHoc, request::FlashMessage, routes, Build};

mod auth;
mod config;
mod database;
pub mod models;
mod routes;
mod schema;

#[get("/")]
fn index(flash: Option<FlashMessage<'_>>) -> String {
    if let Some(f) = flash {
        f.message().to_owned()
    } else {
        "Hello World".to_string()
    }
}

pub fn rocket() -> rocket::Rocket<Build> {
    dotenv().ok();

    rocket::custom(config::from_env())
        .attach(database::Db::fairing())
        .attach(AdHoc::on_ignite(
            "Database migrations",
            database::run_migrations,
        ))
        .mount("/api", routes![index])
        .mount("/api/problems", routes::problems::routes())
        .mount("/api/users", routes::users::routes())
}
