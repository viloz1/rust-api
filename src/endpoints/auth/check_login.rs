use rocket::post;
use rocket::serde::{json::Json, Serialize};
use rocket_auth::*;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    user: Option<User>,
}

#[post("/check_login")]
pub async fn check_login(auth: Auth<'_>) -> Json<Task> {
    Json(Task {
        user: auth.get_user().await,
    })
}
