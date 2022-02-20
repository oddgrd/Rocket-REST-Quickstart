use super::QueryResult;
use crate::{
    models::problem::{NewProblem, Problem},
    schema::problems,
    DbPool,
};
use diesel::prelude::*;
use rocket::{response::status::Created, serde::json::Json};

#[post("/problems", format = "json", data = "<new_problem>")]
pub async fn create_problem(
    db: DbPool,
    new_problem: Json<NewProblem>,
) -> QueryResult<Created<Json<Problem>>> {
    let values = new_problem.clone();
    let problem: Problem = db
        .run(move |conn| {
            diesel::insert_into(problems::table)
                .values(values)
                .get_result(conn)
        })
        .await?;

    let location = uri!("/api", get_problem(problem.id));
    Ok(Created::new(location.to_string()).body(Json(problem)))
}

#[get("/problems")]
pub async fn get_problems(db: DbPool) -> QueryResult<Json<Vec<Problem>>> {
    let problems: Vec<Problem> = db
        .run(move |conn| {
            problems::table
                .order_by(problems::created_at.desc())
                .load(conn)
        })
        .await?;

    Ok(Json(problems))
}

#[get("/problems/<id>")]
pub async fn get_problem(db: DbPool, id: i32) -> Option<Json<Problem>> {
    db.run(move |conn| problems::table.filter(problems::id.eq(id)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[put("/problems/<id>", format = "json", data = "<new_problem>")]
pub async fn update_problem(
    db: DbPool,
    id: i32,
    new_problem: Json<NewProblem>,
) -> QueryResult<Json<Problem>> {
    let values = new_problem.clone();
    let updated_row = db
        .run(move |conn| {
            diesel::update(problems::table)
                .filter(problems::id.eq(id))
                .set(values)
                .get_result(conn)
        })
        .await?;

    Ok(Json(updated_row))
}

#[delete("/problems/<id>")]
pub async fn delete_problem(db: DbPool, id: i32) -> QueryResult<Option<()>> {
    let affected = db
        .run(move |conn| {
            diesel::delete(problems::table)
                .filter(problems::id.eq(id))
                .execute(conn)
        })
        .await?;

    Ok((affected == 1).then(|| ()))
}
