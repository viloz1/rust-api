//! Authorization of the website.
//! Makes sure that only authorized clients are
//! allowed to connect to certain parts.

use rocket:: post;
use rocket::fairing::AdHoc;
use rocket_dyn_templates::{context,Template};
use rocket::form::Form;
use rocket::response::{Redirect};
use rocket_auth::{*};
use std::time;

/// If a client requests the /login page, return login.html
/// with no context.
#[get("/login")]
fn login() -> Template {
    Template::render("login", context!())
}

/// If a client sends the login form, call this function. If the
/// login is succesfull, redirect the logged in user to /.
/// Otherwise, redirect to /login to try again.
/// 
/// The user will also only stay logged in for one hour.
#[post("/login", data = "<form>")]
async fn post_login(auth: Auth<'_>, form: Form<Login>) -> Redirect {
    let one_hour = time::Duration::from_secs(60 * 60);
    let result = auth.login_for(&form, one_hour).await;
    println!("login attempt: {:?}", result);
    match result {
        Err(_) => Redirect::to("/login"),
        Ok(_) => Redirect::to("/")
    }
}

/// Stage auth. Used in attach in the main Rocket launch. This is
/// to make sure that Rocket manages the requests in this module.
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth", |rocket| async {
        rocket.mount("/", routes![login, post_login])
    })
}