use super::DbResult;
use crate::diesel::prelude::*;
use crate::models::user::HashedPassword;
use crate::{
    auth::Auth,
    database::Db,
    models::user::{Login, NewUser, Profile, Register, User},
    schema::users,
};

use rocket::{
    form::Form,
    http::{Cookie, CookieJar},
    response::status::{Accepted, Created, Unauthorized},
    routes,
    serde::json::Json,
    uri,
};

#[post("/register", data = "<data>")]
pub async fn register(
    db: Db,
    jar: &CookieJar<'_>,
    data: Form<Register>,
) -> DbResult<Created<Json<User>>> {
    let values = data.into_inner();

    let new_user = NewUser {
        username: values.username,
        email: values.email,
        password_hash: HashedPassword::hash(&values.password),
    };

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

#[post("/login", data = "<data>")]
async fn login(
    db: Db,
    jar: &CookieJar<'_>,
    data: Form<Login>,
) -> DbResult<Accepted<()>, Unauthorized<String>> {
    let values = data.into_inner();

    let user = db
        .run(move |conn| {
            users::table
                .filter(users::username.eq(&values.username))
                .get_result::<User>(conn)
                .map_err(|_| Unauthorized(Some("user doesn't exist".to_string())))
        })
        .await?;

    user.password_hash.verify(&values.password)?;

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
