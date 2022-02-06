use crate::{
    models::problem::{NewProblem, Problem},
    schema, DBPool, QueryResult,
};
use diesel::prelude::*;
use rocket::{
    response::status::{self, Created},
    serde::json::Json,
};

#[post("/problems", format = "json", data = "<new_problem>")]
pub async fn create_problem(
    conn: DBPool,
    new_problem: Json<NewProblem>,
) -> QueryResult<status::Created<Json<Problem>>> {
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
pub async fn get_problem(conn: DBPool, id: i32) -> Option<Json<Problem>> {
    conn.run(move |c| {
        schema::problems::table
            .filter(schema::problems::id.eq(id))
            .first(c)
    })
    .await
    .map(Json)
    .ok()
}
