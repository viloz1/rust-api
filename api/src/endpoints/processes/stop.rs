use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::State;

use crate::communication::protocols::{
    From, Request, RequestResult, RequestType,
};
use crate::endpoints::wait_response;
use crate::guards::timer::TimerRequest;
use crate::states::ProcessComm;
use crate::states::Timeout;
use crossbeam::channel::unbounded;
use rocket_auth::User;

#[post("/stop/<id>")]
pub fn stop<'a>(
    _auth: User,
    id: usize,
    state: &State<ProcessComm>,
    timeout: &State<Timeout>,
    _time: TimerRequest,
) -> Custom<&'a str> {
    let (tx, rx) = unbounded::<RequestResult>();

    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::Stop,
        id: Some(id),
        answer_channel: Some(tx),
        ..Default::default()
    });

    match result {
        Err(_) => return Custom(Status::InternalServerError, ""),
        _ => (),
    };

    return wait_response(timeout.timeout, rx);
}
