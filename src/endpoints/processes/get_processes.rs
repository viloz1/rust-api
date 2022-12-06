use std::collections::HashMap;
use std::hash::Hash;

use crossbeam::channel::unbounded;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::State;

use crate::communication::protocols::{
    From, Request, RequestResult, RequestResultStatus, RequestType,
};
use crate::states::processcomm::ProcessComm;
use rocket_auth::User;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    processes: Vec<HashMap<String, String>>,
}

#[get("/get_processes")]
pub async fn get_processes(auth: User, state: &State<ProcessComm>) -> Custom<Option<Json<Task>>> {
    let (tx, rx) = unbounded();
    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::GetProcesses,
        answer_channel: Some(tx),
        ..Default::default()
    });
    match result {
        Err(_) => return Custom(Status::InternalServerError, None),
        _ => (),
    };
    let answer = rx.recv();
    match answer {
        Ok(RequestResult {
            status: RequestResultStatus::Success,
            body: Some(body),
            id: _,
            process_status: _,
        }) => {
            return Custom(
                Status::Ok,
                Some(
                    (Json(Task {
                        processes: procstring_as_list(body),
                    })),
                ),
            )
        }
        _ => return Custom(Status::InternalServerError, None),
    };
}

fn procstring_as_list(str: String) -> Vec<HashMap<String, String>> {
    let mut rv_vec: Vec<HashMap<String, String>> = Vec::new();
    let split: Vec<&str> = str.split(":").collect();
    for s in split {
        let mut hash: HashMap<String, String> = HashMap::new();
        let types: Vec<&str> = s.split(",").collect();
        hash.insert("name".to_string(), types[0].to_string());
        hash.insert("id".to_string(), types[1].to_string());
        hash.insert("status".to_string(), types[2].to_string());
        rv_vec.push(hash);
    }
    return rv_vec;
}
