use crate::schema::users;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use chrono::{DateTime, Utc};
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use rocket::{
    form::{self, Error, FromForm},
    response::status::Unauthorized,
};
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
pub struct Register {
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
    pub fn hash(password: &str) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .expect("hash error")
            .to_string();

        Self(password_hash)
    }

    pub fn verify(&self, input_password: &str) -> Result<(), Unauthorized<String>> {
        let parsed_hash = PasswordHash::new(&self.0).expect("hash error");
        Argon2::default()
            .verify_password(input_password.as_bytes(), &parsed_hash)
            .map_err(|_| Unauthorized(Some("invalid password".to_string())))?;

        Ok(())
    }
}

impl<DB: Backend<RawValue = [u8]>> FromSql<diesel::sql_types::Text, DB> for HashedPassword {
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Pg>>::from_sql(bytes).map(HashedPassword)
    }
}

impl ToSql<diesel::sql_types::Text, Pg> for HashedPassword {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <String as ToSql<diesel::sql_types::Text, Pg>>::to_sql(&self.0, out)
    }
}
