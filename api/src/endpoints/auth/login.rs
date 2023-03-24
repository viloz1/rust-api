//! Authorization of the website.
//! Makes sure that only authorized clients are
//! allowed to connect to certain parts.

use actix_web::{http::header::ContentType, post, HttpResponse, web, cookie::Cookie};
use serde::Deserialize;

use crate::guards::auth::users::UserManager;

#[derive(Deserialize, Clone)]
struct LoginForm {
    username: String,
    password: String
}

#[post("/login")]
pub async fn login(info: web::Json<LoginForm>, users: web::Data<UserManager>) -> HttpResponse {
    println!("one req");
    match users.login(info.username.clone(), info.password.clone()){
        Ok(session) => {
            println!("{} --END", session.auth_key);
            
            let cookie = Cookie::build("viloz-auth", session.to_string()).path("/").domain("localhost").finish();
            let res = HttpResponse::Ok().cookie(cookie).body("");
            println!("{:?}", session.to_string());
            res
        },
        Err(_) => HttpResponse::BadRequest().body("")
    }
}
