//! Authorization of the website.
//! Makes sure that only authorized clients are
//! allowed to connect to certain parts.

use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket_auth::*;
use std::time;

#[post("/logout")]
pub async fn logout(auth: Auth<'_>) -> Status {
    let one_hour = time::Duration::from_secs(60 * 60);
    let result = auth.logout();
    match result {
        Err(_) => Status::BadRequest,
        Ok(_) => Status::Accepted,
    }
}
