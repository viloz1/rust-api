use actix_web::{post, Responder, Result, web};
use serde::Serialize;

use crate::guards::auth::auth::Auth;

type User = String;

#[derive(Serialize)]
pub struct Task {
    user: Option<User>,
}

#[post("/check_login")]
pub async fn check_login(auth: Auth) -> Result<impl Responder> {
    Ok(web::Json(auth.user))
}
