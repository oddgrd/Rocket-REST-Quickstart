use crate::{models::problem::Problem, schema::problems};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "problems"]
pub struct NewProblem<'a> {
    pub title: &'a str,
    pub grade: i32,
    pub rating: i32,
}

pub fn create(conn: &PgConnection, title: &str, grade: i32, rating: i32) -> QueryResult<Problem> {
    let new_problem = &NewProblem {
        title,
        grade,
        rating,
    };
    diesel::insert_into(problems::table)
        .values(new_problem)
        .get_result::<Problem>(conn)
}

pub fn find_one(conn: &PgConnection, title: &str) -> Option<Problem> {
    let problem = problems::table
        .filter(problems::title.eq(title))
        .first::<Problem>(conn)
        .map_err(|err| eprintln!("problem::find_one: {}", err))
        .ok()?;

    Some(problem)
}
