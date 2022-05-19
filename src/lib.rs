#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate rocket_sync_db_pools;

mod auth;
mod configuration;
mod database;
mod models;
mod routes;
mod schema;
pub mod startup;
