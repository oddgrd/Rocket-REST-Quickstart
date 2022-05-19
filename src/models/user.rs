use crate::schema::users;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use rocket::form::{self, Error, FromForm};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: HashedPassword,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn to_profile(&self) -> Profile {
        Profile {
            id: self.id,
            username: self.username.clone(),
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
pub struct Login<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

#[derive(FromForm)]
pub struct RegisterData {
    #[field(validate = len(2..=25))]
    pub username: String,
    #[field(validate = validate_email())]
    pub email: String,
    #[field(validate = len(10..=50))]
    pub password: String,
}

fn validate_email<'v>(email: &str) -> form::Result<'v, ()> {
    if !validator::validate_email(email) {
        return Err(Error::validation("invalid email").into());
    }

    Ok(())
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: HashedPassword,
}

// Implementation note:
// To make HashedPassword insertable or queryable with Diesel, the traits AsExpression,
// FromSqlRow, FromSql and ToSql have to be implemented to serialize/deserialize
// HashedPassword into its SQL-type: Text.
#[derive(AsExpression, FromSqlRow, Debug)]
#[sql_type = "diesel::sql_types::Text"]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn parse(raw_password: &str) -> anyhow::Result<HashedPassword> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default().hash_password(raw_password.as_bytes(), &salt)?;

        Ok(Self(password_hash.to_string()))
    }

    pub fn verify(&self, raw_password: &str) -> anyhow::Result<()> {
        let parsed_hash = argon2::PasswordHash::new(&self.0)?;
        Argon2::default().verify_password(raw_password.as_bytes(), &parsed_hash)?;

        Ok(())
    }
}

impl<DB> FromSql<diesel::sql_types::Text, DB> for HashedPassword
where
    DB: Backend,
    String: FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        <String>::from_sql(bytes).map(HashedPassword)
    }
}

impl<DB> ToSql<diesel::sql_types::Text, DB> for HashedPassword
where
    DB: Backend,
    String: ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        <String>::to_sql(&self.0, out)
    }
}
