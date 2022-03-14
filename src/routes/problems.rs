use super::DbResult;
use crate::{
    auth::Auth,
    database::Db,
    models::problem::{NewProblem, Problem, ProblemData, UpdateProblem},
    schema::problems,
};
use diesel::prelude::*;
use rocket::{form::Form, response::status::Created, routes, serde::json::Json, uri};

#[post("/", data = "<data>")]
pub async fn create_problem(
    db: Db,
    data: Form<ProblemData>,
    user: Auth,
) -> DbResult<Created<Json<Problem>>> {
    let values = data.into_inner();
    let new_problem = NewProblem::new(values.title, values.grade, user.0);

    let problem: Problem = db
        .run(move |conn| {
            diesel::insert_into(problems::table)
                .values(new_problem)
                .get_result(conn)
        })
        .await?;

    let location = uri!("/api/problems/", get_problem(problem.id));
    Ok(Created::new(location.to_string()).body(Json(problem)))
}

#[get("/")]
pub async fn get_problems(db: Db) -> DbResult<Json<Vec<Problem>>> {
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

#[put("/<id>", data = "<data>")]
pub async fn update_problem(
    db: Db,
    user: Auth,
    id: i32,
    data: Form<ProblemData>,
) -> DbResult<Json<Problem>> {
    let values = data.into_inner();
    let update_problem = UpdateProblem {
        title: values.title,
        grade: values.grade,
    };

    let updated_problem = db
        .run(move |conn| {
            diesel::update(problems::table)
                .filter(problems::id.eq(id))
                .filter(problems::creator.eq(user.0))
                .set(&update_problem)
                .get_result(conn)
        })
        .await?;

    Ok(Json(updated_problem))
}

#[delete("/<id>")]
pub async fn delete_problem(db: Db, user: Auth, id: i32) -> DbResult<Option<()>> {
    let affected = db
        .run(move |conn| {
            diesel::delete(problems::table)
                .filter(problems::id.eq(id))
                .filter(problems::creator.eq(user.0))
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
