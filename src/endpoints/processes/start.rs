use rocket::http::Status;
use rocket::response::status::{Accepted, BadRequest};
use rocket::State;

use crate::communication::protocols::{none_request, From, Request, RequestType};
use crate::guards::timer::TimerRequest;
use crate::states::processcomm::ProcessComm;
use rocket_auth::User;

#[post("/start/<id>")]
pub fn start(
    auth: User,
    id: usize,
    state: &State<ProcessComm>,
    time: TimerRequest,
) -> Result<Accepted<String>, BadRequest<String>> {
    println!("Starting");
    let empty = none_request();
    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::Start,
        id: Some(id),
        ..empty
    });
    println!("{:?}",result);
    match result {
        Err(_) => Err(BadRequest(Some("a".to_string()))),
        _ => Ok(Accepted(Some("h".to_string()))),
    }
}
