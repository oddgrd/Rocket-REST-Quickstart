use crate::schema::problems;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "problems"]
pub struct NewProblem<'a> {
    pub title: &'a str,
    pub grade: i16,
    pub rating: i16,
}
