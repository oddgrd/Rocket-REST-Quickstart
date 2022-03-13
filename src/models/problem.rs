use crate::schema::problems;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    pub id: i32,
    pub title: String,
    pub grade: i32,
    pub rating: Option<i32>,
    pub creator: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromForm)]
pub struct ProblemData {
    pub title: String,
    pub grade: i32,
}

#[derive(Insertable)]
#[table_name = "problems"]
pub struct NewProblem {
    pub title: String,
    pub grade: i32,
    pub creator: i32,
}

impl NewProblem {
    pub fn new(title: String, grade: i32, creator: i32) -> Self {
        NewProblem {
            title,
            grade,
            creator,
        }
    }
}

#[derive(AsChangeset)]
#[table_name = "problems"]
pub struct UpdateProblem {
    pub title: String,
    pub grade: i32,
}
