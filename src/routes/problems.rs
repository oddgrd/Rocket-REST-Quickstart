use super::QueryResult;
use crate::{
    models::problem::{NewProblem, Problem},
    schema, DbPool,
};
use diesel::prelude::*;
use rocket::{response::status::Created, serde::json::Json};

#[post("/problems", format = "json", data = "<new_problem>")]
pub async fn create_problem(
    conn: DbPool,
    new_problem: Json<NewProblem>,
) -> QueryResult<Created<Json<Problem>>> {
    let values = new_problem.clone();
    let problem: Problem = conn
        .run(move |c| {
            diesel::insert_into(schema::problems::table)
                .values(values)
                .get_result(c)
        })
        .await?;

    let location = uri!("/api", get_problem(problem.id));
    Ok(Created::new(location.to_string()).body(Json(problem)))
}

#[get("/problems/<id>")]
pub async fn get_problem(conn: DbPool, id: i32) -> Option<Json<Problem>> {
    conn.run(move |c| {
        schema::problems::table
            .filter(schema::problems::id.eq(id))
            .first(c)
    })
    .await
    .map(Json)
    .ok()
}
