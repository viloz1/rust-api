//! Authorization of the website.
//! Makes sure that only authorized clients are
//! allowed to connect to certain parts.

use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket_auth::*;
use std::time;

#[post(
    "/login",
    format = "application/x-www-form-urlencoded",
    data = "<form>"
)]
pub async fn login(auth: Auth<'_>, form: Json<Login>) -> Status {
    let one_hour = time::Duration::from_secs(60 * 60);
    let result = auth.login_for(&form, one_hour).await;
    info!("login attempt: {:?}", result);
    match result {
        Err(_) => Status::NotAcceptable,
        Ok(_) => Status::Accepted,
    }
}
