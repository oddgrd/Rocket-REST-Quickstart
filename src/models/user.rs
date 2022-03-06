use crate::schema::users;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Profile {
    id: i32,
    username: String,
    created_at: DateTime<Utc>,
}

#[derive(FromForm)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(FromForm)]
pub struct Register {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn to_profile(self) -> Profile {
        Profile {
            id: self.id,
            username: self.username,
            created_at: self.created_at,
        }
    }
}

impl NewUser {
    pub fn new(username: String, email: String, password: String) -> NewUser {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .expect("hash error")
            .to_string();

        NewUser {
            username,
            email,
            password_hash,
        }
    }
}
