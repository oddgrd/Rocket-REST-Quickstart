use dotenv::dotenv;
use rocket::{fairing::AdHoc, routes, Build};

use crate::configuration;
use crate::database;
use crate::routes;

pub fn rocket() -> rocket::Rocket<Build> {
    dotenv().ok();

    rocket::custom(configuration::from_env())
        .attach(database::Db::fairing())
        .attach(AdHoc::on_ignite(
            "Database migrations",
            database::run_migrations,
        ))
        .mount("/api", routes![routes::health_check::health_check])
        .mount("/api/problems", routes::problems::routes())
        .mount("/api/users", routes::users::routes())
}
