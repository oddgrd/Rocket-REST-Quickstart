use crate::database;
use crate::routes;
extern crate dotenv;
use rocket::figment::Figment;
use rocket::{fairing::AdHoc, routes, Build};

pub fn rocket(configuration: Figment) -> rocket::Rocket<Build> {
    rocket::custom(configuration)
        .attach(database::Db::fairing())
        .attach(AdHoc::on_ignite(
            "Database migrations",
            database::run_migrations,
        ))
        .mount("/api", routes![routes::health_check::health_check])
        .mount("/api/problems", routes::problems::routes())
        .mount("/api/users", routes::users::routes())
}
