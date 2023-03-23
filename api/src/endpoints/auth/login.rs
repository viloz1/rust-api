//! Authorization of the website.
//! Makes sure that only authorized clients are
//! allowed to connect to certain parts.

use actix_web::{http::header::ContentType, post, HttpResponse};

#[post("/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("")
}
