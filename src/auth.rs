use rocket::{
    outcome::IntoOutcome,
    request::{self, FromRequest, Request},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Auth(pub i32);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Auth, Self::Error> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(Auth)
            .or_forward(())
    }
}
