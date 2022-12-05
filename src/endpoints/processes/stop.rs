use std::{time, thread};

use rocket::State;
use rocket::response::status::Custom;
use rocket::http::Status;

use crate::communication::protocols::{From, Request, RequestType, RequestResultStatus, RequestResult};
use crate::guards::timer::TimerRequest;
use crate::states::Timeout;
use crate::states::processcomm::ProcessComm;
use crossbeam::channel::unbounded;
use rocket_auth::User;

#[post("/stop/<id>")]
pub fn stop(auth: User, id: usize, state: &State<ProcessComm>, timeout: &State<Timeout>, time: TimerRequest) -> Custom<Option<String>> {

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
    
    let t = 0;
    while t < timeout.timeout*2 {
        let answer = rx.recv().unwrap();
        match answer {
            RequestResult {
                status: RequestResultStatus::Success,
                process_status: _,
                message_id: _,
                id: _,
                body: _

            } => {
                return Custom(Status::Ok, None)
            },
            RequestResult {
                status: RequestResultStatus::Failed,
                process_status: _,
                message_id: _,
                id: _,
                body: body

            } => {
                return Custom(Status::InternalServerError, Some(body))
            },
            _ => {
                ()
            }
        }
        t += 1;
        thread::sleep(time::Duration::from_millis(500))
    };
    return Custom(Status::InternalServerError, None)
}
