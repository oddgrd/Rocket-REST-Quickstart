# Rocket Postgres API quickstart-template (WIP)

The aim of this project is to create a quickstart-template for a Rocket REST-API with some route examples, authentication and database-setup (in this case Postgres with Diesel.rs ORM). As I am new to Rust and Rocket, this is also an educational project.

## Testing

Launch a Docker postgres service with `./scripts/init_db.sh`, then run tests with `cargo test`.
A new database is created for each test run. 

Install Docker: https://docs.docker.com/get-docker/.
