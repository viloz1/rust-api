use actix_web::{web};
use auth::*;
use processes::*;

mod auth;
mod processes;

pub fn add_services() -> actix_web::Scope {

    let processes = web::scope("/processes")
    .service(start::start)
    .service(stop::stop)
    .service(restart::restart)
    .service(restartpull::restartpull)
    .service(get_processes::get_processes);

    let auth = web::scope("/auth")
    .service(login::login)
    .service(check_login::check_login)
    .service(logout::logout);

    return web::scope("/api")
    .service(auth)
    .service(processes);
    
}