//! Authorization of the website.
//! Makes sure that only authorized clients are
//! allowed to connect to certain parts.

use rocket::http::Status;
use rocket::post;
use rocket_auth::*;

#[post("/logout")]
pub async fn logout(auth: Auth<'_>) -> Status {
    let result = auth.logout();
    match result {
        Err(_) => Status::BadRequest,
        Ok(_) => Status::Accepted,
    }
}
