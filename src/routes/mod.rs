use rocket::response::Debug;

pub mod problems;

pub type QueryResult<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
