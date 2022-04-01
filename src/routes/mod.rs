use rocket::response::Debug;

pub mod health_check;
pub mod problems;
pub mod users;

pub type DbResult<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
