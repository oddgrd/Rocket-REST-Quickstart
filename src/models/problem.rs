use crate::schema::problems;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
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
pub struct ProblemData<'r> {
    #[field(validate = len(2..=25))]
    pub title: &'r str,
    #[field(validate = range(0..=19))]
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
    pub fn new(title: &str, grade: i32, creator: i32) -> Self {
        NewProblem {
            title: title.into(),
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
