use chrono::{DateTime, Utc};

#[derive(Queryable)]
pub struct Problem {
    pub id: i32,
    pub title: String,
    pub grade: u8,
    pub rating: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
