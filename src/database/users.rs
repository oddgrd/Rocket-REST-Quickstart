use crate::diesel::prelude::*;
use crate::{
    database::Db,
    models::user::{NewUser, User},
    schema::users,
};

/// Persist new user in database.
pub async fn insert_user(db: Db, values: NewUser) -> Result<User, diesel::result::Error> {
    let user: User = db
        .run(move |conn| {
            diesel::insert_into(users::table)
                .values(values)
                .get_result(conn)
        })
        .await?;

    Ok(user)
}

/// Get user from database if user exists
pub async fn find_user(db: Db, username: String) -> Result<Option<User>, diesel::result::Error> {
    let user = db
        .run(move |conn| {
            users::table
                .filter(users::username.eq(&username))
                .get_result::<User>(conn)
                .optional()
        })
        .await?;

    Ok(user)
}
