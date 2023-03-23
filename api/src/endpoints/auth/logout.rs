//! Authorization of the website.
//! Makes sure that only authorized clients are
//! allowed to connect to certain parts.

use actix_web::{post, HttpResponse};

#[post("/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("")
}
