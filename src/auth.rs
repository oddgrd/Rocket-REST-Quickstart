use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest, Request},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Auth(pub i32);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Auth, Self::Error> {
        let parsed_cookie = request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(Auth);

        if let Some(auth) = parsed_cookie {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}
