//! Authorization of the website.
//! Makes sure that only authorized clients are
//! allowed to connect to certain parts.

use actix_web::{http::header::ContentType, post, HttpResponse, web};
use serde::Deserialize;

use crate::guards::auth::users::UserManager;

#[derive(Deserialize, Clone)]
struct LoginForm {
    username: String,
    password: String
}

#[post("/login")]
pub async fn login(info: web::Json<LoginForm>, users: web::Data<UserManager>) -> HttpResponse {
    match users.login(info.username.clone(), info.password.clone()){
        Ok(_) => HttpResponse::Ok().body(""),
        Err(_) => HttpResponse::BadRequest().body("")
    }
}
