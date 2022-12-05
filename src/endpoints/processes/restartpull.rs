use rocket::http::Status;
use rocket::State;

use crate::communication::protocols::{From, Request, RequestType};
use crate::guards::timer::TimerRequest;
use crate::states::processcomm::ProcessComm;
use rocket_auth::User;

#[post("/restartpull/<id>")]
pub fn restartpull(
    auth: User,
    id: usize,
    state: &State<ProcessComm>,
    time: TimerRequest,
) -> Status {
    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::RestartPull,
        id: Some(id),
        ..Default::default()
    });
    match result {
        Err(_) => Status::InternalServerError,
        _ => Status::Ok,
    }
}
