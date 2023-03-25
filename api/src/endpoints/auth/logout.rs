//! Authorization of the website.
//! Makes sure that only authorized clients are
//! allowed to connect to certain parts.

use actix_web::{post, HttpResponse, HttpRequest, web};

use crate::guards::auth::{cookie::get_session, users::UserManager};

#[post("/logout")]
pub async fn logout(req: HttpRequest, users: web::Data<UserManager>) -> HttpResponse {
    if let Some(cookie) = req.cookie("viloz-auth") {
        if let Ok(session) = get_session(cookie) {
            users.logout(session.id);
        }
    }
    HttpResponse::Ok().body("")
}
