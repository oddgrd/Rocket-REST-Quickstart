use crate::{
    db::{problems, DbConnection},
    models::problem::Problem,
};
use diesel::result::Error;
use rocket::{http::Status, response::status, serde::json::Json};

#[get("/<name>")]
pub fn greeting(name: String) -> String {
    format!("Hello, {}!", name)
}

#[post("/problems", format = "json", data = "<new_problem>")]
pub fn post_problem(
    new_problem: Json<problems::NewProblem>,
    conn: DbConnection,
) -> Result<status::Created<Json<Problem>>, Status> {
    let new_problem = new_problem.into_inner();
    problems::create(
        &conn,
        &new_problem.title,
        new_problem.grade,
        new_problem.rating,
    )
    .map(|problem| problem_created(problem))
    .map_err(|error| error_status(error))
}

fn problem_created(problem: Problem) -> status::Created<Json<Problem>> {
    status::Created::new("location URI here".to_string()).body(Json(problem))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
