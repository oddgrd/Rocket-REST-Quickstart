pub mod problems;

use diesel::{pg::PgConnection, r2d2, r2d2::ConnectionManager};
use dotenv::dotenv;
use rocket::{http::Status, request, request::FromRequest, request::Outcome, Request, State};
use std::env;
use std::ops::Deref;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("Failed to create pool.")
}

fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABUSE_URL must be set")
}

pub struct DbConnection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConnection {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<DbConnection, Self::Error> {
        let pool = request.guard::<&State<Pool>>().await;
        match pool.unwrap().get() {
            Ok(conn) => Outcome::Success(DbConnection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConnection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
