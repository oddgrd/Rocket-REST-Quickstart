use super::QueryResult;
use crate::diesel::prelude::*;
use crate::{
    auth::Auth,
    database::Db,
    models::user::{Login, NewUser, Profile, Register, User},
    schema::users,
};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use rocket::{
    form::Form,
    http::{Cookie, CookieJar},
    response::{
        status::{Created, Unauthorized},
        Redirect,
    },
    serde::json::Json,
};

#[post("/register", data = "<register>")]
pub async fn register(
    db: Db,
    jar: &CookieJar<'_>,
    register: Form<Register>,
) -> QueryResult<Created<Json<User>>> {
    let values = register.into_inner();

    // Hashes password
    let new_user = NewUser::new(values.username, values.email, values.password);

    let user: User = db
        .run(move |conn| {
            diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(conn)
        })
        .await?;

    jar.add_private(Cookie::new("user_id", user.id.to_string()));

    let location = uri!("/api/users/", get_profile(user.id));
    Ok(Created::new(location.to_string()).body(Json(user)))
}

#[post("/login", data = "<login>")]
async fn login(
    db: Db,
    jar: &CookieJar<'_>,
    login: Form<Login>,
) -> Result<Redirect, Unauthorized<String>> {
    let values = login.into_inner();

    let user = db
        .run(move |conn| {
            users::table
                .filter(users::username.eq(&values.username))
                .get_result::<User>(conn)
                .map_err(|_| Unauthorized(Some("user doesn't exist".to_string())))
        })
        .await?;

    let parsed_hash = PasswordHash::new(&user.password_hash).expect("hash error");
    Argon2::default()
        .verify_password(&values.password.as_bytes(), &parsed_hash)
        .map_err(|_| Unauthorized(Some("invalid password".to_string())))?;

    jar.add_private(Cookie::new("user_id", user.id.to_string()));
    Ok(Redirect::to("/api"))
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

pub fn routes() -> Vec<rocket::Route> {
    routes![login, register, me, get_profile]
}
