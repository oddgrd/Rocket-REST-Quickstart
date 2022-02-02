#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket::Build;

mod db;
mod models;
mod routes;
mod schema;

pub fn rocket() -> rocket::Rocket<Build> {
    rocket::build().mount("/api", routes![routes::problems::greeting])
}
