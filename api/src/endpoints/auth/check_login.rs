use actix_web::{post, Responder, Result, web};
use serde::Serialize;

type User = String;

#[derive(Serialize)]
pub struct Task {
    user: Option<User>,
}

#[post("/check_login")]
pub async fn check_login() -> Result<impl Responder> {
    let user = Task {
        user: Some("viloz".to_string())
    };
    Ok(web::Json(user))
}
