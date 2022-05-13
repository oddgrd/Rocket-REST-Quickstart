use crate::schema::users;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use regex::Regex;
use rocket::form::{self, Error, FromForm};
use serde::{Deserialize, Serialize};

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

impl User {
    pub fn to_profile(self) -> Profile {
        Profile {
            id: self.id,
            username: self.username,
            created_at: self.created_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub id: i32,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

#[derive(FromForm)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(FromForm)]
pub struct Register<'r> {
    #[field(validate = len(2..=25))]
    pub username: &'r str,
    #[field(validate = validate_email())]
    pub email: &'r str,
    #[field(validate = len(10..=50))]
    pub password: &'r str,
}

fn validate_email<'v>(email: &str) -> form::Result<'v, ()> {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();

    if !email_regex.is_match(email) {
        return Err(Error::validation("invalid email").into());
    }

    Ok(())
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl NewUser {
    pub fn new(username: &str, email: &str, password: &str) -> NewUser {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .expect("hash error")
            .to_string();

        NewUser {
            username: username.into(),
            email: email.into(),
            password_hash,
        }
    }
}
