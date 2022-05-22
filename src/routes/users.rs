use crate::database::users::{find_user, insert_user};
use crate::diesel::prelude::*;
use crate::models::user::{HashedPassword, NewUser};
use crate::{
    auth::Auth,
    database::Db,
    models::user::{Login, Profile, RegisterData, User},
    schema::users,
};

use anyhow::Context;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

use rocket::http::Status;
use rocket::response::Responder;
use rocket::{
    form::Form,
    http::{Cookie, CookieJar},
    response::status::{Accepted, Created},
    routes,
    serde::json::Json,
    uri,
};
use rocket::{request::Request, response::status::Custom};

#[derive(thiserror::Error, Debug)]
pub enum UserError {
    #[error("{0}")]
    DuplicatedUser(&'static str),
    #[error("There is no user associated with the provided username.")]
    UnknownUser,
    #[error("Invalid password.")]
    InvalidPassword,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl<'r> Responder<'r, 'static> for UserError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            UserError::DuplicatedUser(msg) => {
                Custom(Status::UnprocessableEntity, msg).respond_to(req)
            }
            UserError::UnknownUser => {
                Custom(Status::NotFound, "User doesn't exist").respond_to(req)
            }
            UserError::InvalidPassword => Status::Unauthorized.respond_to(req),
            _ => Status::InternalServerError.respond_to(req),
        }
    }
}

impl From<DieselError> for UserError {
    fn from(err: DieselError) -> UserError {
        if let DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("users_username_key") => {
                    UserError::DuplicatedUser("A user with that username already exists")
                }
                Some("users_email_key") => {
                    UserError::DuplicatedUser("A user with that email already exists")
                }
                _ => UserError::UnexpectedError(err.into()),
            }
        } else {
            UserError::UnexpectedError(err.into())
        }
    }
}

impl TryFrom<RegisterData> for NewUser {
    type Error = anyhow::Error;

    fn try_from(values: RegisterData) -> Result<NewUser, Self::Error> {
        let password_hash = HashedPassword::parse(&values.password)?;
        Ok(Self {
            username: values.username,
            email: values.email,
            password_hash,
        })
    }
}

#[post("/register", data = "<data>")]
pub async fn register(
    db: Db,
    jar: &CookieJar<'_>,
    data: Form<RegisterData>,
) -> Result<Created<Json<User>>, UserError> {
    let values = data.into_inner();

    let new_user: NewUser = values.try_into().context("Failed to hash password.")?;

    let user = insert_user(db, new_user).await?;

    jar.add_private(Cookie::new("user_id", user.id.to_string()));

    let location = uri!("/api/users/", get_profile(user.id));
    Ok(Created::new(location.to_string()).body(Json(user)))
}

#[post("/login", data = "<data>")]
async fn login(
    db: Db,
    jar: &CookieJar<'_>,
    data: Form<Login<'_>>,
) -> Result<Accepted<()>, UserError> {
    let user = find_user(db, data.username.into())
        .await
        .context("User with this username doesn't exist")?
        .ok_or(UserError::UnknownUser)?;

    user.password_hash
        .verify(data.password)
        .map_err(|_| UserError::InvalidPassword)?;

    jar.add_private(Cookie::new("user_id", user.id.to_string()));
    Ok(Accepted::<()>(None))
}

#[get("/me")]
async fn me(db: Db, user: Auth) -> Option<Json<User>> {
    db.run(move |conn| users::table.filter(users::id.eq(&user.0)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[get("/<id>")]
async fn get_profile(db: Db, id: i32) -> Option<Json<Profile>> {
    let user: User = db
        .run(move |conn| users::table.filter(users::id.eq(&id)).first(conn))
        .await
        .ok()?;

    Some(Json(user.to_profile()))
}

#[post("/logout")]
fn logout(jar: &CookieJar<'_>) {
    jar.remove_private(Cookie::named("user_id"));
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login, register, me, get_profile, logout]
}
