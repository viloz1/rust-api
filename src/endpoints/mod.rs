use auth::*;
use processes::*;
use rocket::fairing::AdHoc;

mod auth;
mod processes;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("API", |rocket| async {
        rocket
            .mount(
                "/api/auth",
                routes![check_login::check_login, login::login, logout::logout],
            )
            .mount(
                "/api/processes",
                routes![
                    get_processes::get_processes,
                    github::github,
                    stop::stop,
                    start::start,
                    restart::restart,
                    restartpull::restartpull
                ],
            )
    })
}
