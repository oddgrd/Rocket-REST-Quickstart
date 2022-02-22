# Rocket Postgres API quickstart-template

The aim of this project is to create a quickstart-template for a Rocket REST-API with some route examples, authentication and database-setup (in this case Postgres with Diesel.rs ORM). As I am new to Rust and Rocket, this is also an educational project.

Loosely based on [realworld-rust-rocket](https://github.com/TatriX/realworld-rust-rocket), rocket docs and examples. Unlike realworld-rust-rocket, I use Rocket 0.5.0-rc-1 on Rust stable release.

## Testing

Run tests (single-threaded for now) against a temporary Postgres docker container with `./scripts/test.sh` from the root of the source tree.


