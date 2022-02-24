use super::QueryResult;
use crate::{
    database::Db,
    models::problem::{NewProblem, Problem},
    schema::problems,
};
use diesel::prelude::*;
use rocket::{response::status::Created, serde::json::Json};

#[post("/", format = "json", data = "<new_problem>")]
pub async fn create_problem(
    db: Db,
    new_problem: Json<NewProblem>,
) -> QueryResult<Created<Json<Problem>>> {
    let values = new_problem.clone();
    let problem: Problem = db
        .run(move |conn| {
            diesel::insert_into(problems::table)
                .values(&values)
                .get_result(conn)
        })
        .await?;

    let location = uri!("/api", get_problem(problem.id));
    Ok(Created::new(location.to_string()).body(Json(problem)))
}

#[get("/")]
pub async fn get_problems(db: Db) -> QueryResult<Json<Vec<Problem>>> {
    let problems: Vec<Problem> = db
        .run(move |conn| {
            problems::table
                .order_by(problems::created_at.desc())
                .load(conn)
        })
        .await?;

    Ok(Json(problems))
}

#[get("/<id>")]
pub async fn get_problem(db: Db, id: i32) -> Option<Json<Problem>> {
    db.run(move |conn| problems::table.filter(problems::id.eq(id)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[put("/<id>", format = "json", data = "<new_problem>")]
pub async fn update_problem(
    db: Db,
    id: i32,
    new_problem: Json<NewProblem>,
) -> QueryResult<Json<Problem>> {
    let values = new_problem.clone();
    let updated_row = db
        .run(move |conn| {
            diesel::update(problems::table.filter(problems::id.eq(id)))
                .set(&values)
                .get_result(conn)
        })
        .await?;

    Ok(Json(updated_row))
}

#[delete("/<id>")]
pub async fn delete_problem(db: Db, id: i32) -> QueryResult<Option<()>> {
    let affected = db
        .run(move |conn| {
            diesel::delete(problems::table)
                .filter(problems::id.eq(id))
                .execute(conn)
        })
        .await?;

    Ok((affected == 1).then(|| ()))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        create_problem,
        get_problems,
        get_problem,
        update_problem,
        delete_problem
    ]
}
