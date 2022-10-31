use std::collections::HashMap;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::State;

use crate::communication::protocols::{none_request, From, Request, RequestType};
use crate::states::processcomm::ProcessComm;
use rocket_auth::User;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    processes: Vec<HashMap<String, String>>,
}

#[get("/get_processes")]
pub async fn get_processes(auth: User, state: &State<ProcessComm>) -> Result<Json<Task>, Status> {
    let empty = none_request();
    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::GetProcesses,
        ..empty
    });
    match result {
        Err(_) => return Err(Status::InternalServerError),
        _ => {
            let processes_list = state.receiver.recv().unwrap().processes.unwrap();
            Ok(Json(Task {
                processes: processes_list,
            }))
        }
    }
}
