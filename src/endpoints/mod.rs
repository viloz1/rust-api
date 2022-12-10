use std::{thread, time};

use auth::*;
use crossbeam::channel::Receiver;
use processes::*;
use rocket::{fairing::AdHoc, response::status::Custom, http::Status};

use crate::{communication::protocols::{RequestResult, RequestResultStatus}, states::Timeout};

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
                    restartpull::restartpull,
                    create::create
                ],
            )
    })
}

pub fn wait_response(timeout: usize, rx: Receiver<RequestResult>) -> Custom<Option<String>> {
    let mut t = 0;
    while t < timeout * 2 {
        let answer = rx.recv().unwrap();
        match answer {
            RequestResult {
                status: RequestResultStatus::Success,
                body: _,
                process_status: _,
                id: _,
            } => return Custom(Status::Ok, None),
            RequestResult {
                status: RequestResultStatus::Failed,
                body,
                process_status: _,
                id: _,
            } => return Custom(Status::InternalServerError, body),
            _ => (),
        }
        t += 1;
        thread::sleep(time::Duration::from_millis(500))
    }
    return Custom(Status::InternalServerError, None);
}