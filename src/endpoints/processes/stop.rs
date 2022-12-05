use std::{thread, time};

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::State;

use crate::communication::protocols::{
    From, Request, RequestResult, RequestResultStatus, RequestType,
};
use crate::guards::timer::TimerRequest;
use crate::states::processcomm::ProcessComm;
use crate::states::Timeout;
use crossbeam::channel::unbounded;
use rocket_auth::User;

#[post("/stop/<id>")]
pub fn stop(
    auth: User,
    id: usize,
    state: &State<ProcessComm>,
    timeout: &State<Timeout>,
    time: TimerRequest,
) -> Custom<Option<String>> {
    let (tx, rx) = unbounded::<RequestResult>();

    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::Stop,
        id: Some(id),
        answer_channel: Some(tx),
        ..Default::default()
    });

    match result {
        Err(_) => return Custom(Status::InternalServerError, None),
        _ => (),
    };

    let mut t = 0;
    while t < timeout.timeout * 2 {
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
                body: body,
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
