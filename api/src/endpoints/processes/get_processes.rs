use std::collections::HashMap;
use crossbeam::channel::unbounded;

use actix_web::{get, web, Responder, HttpResponse};
use crate::communication::protocols::{
    From, Request, RequestResult, RequestResultStatus, RequestType,
};
use crate::states::ProcessComm;
use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    processes: Vec<HashMap<String, String>>,
}

#[get("/get_processes")]
pub async fn get_processes(state: web::Data<ProcessComm>) -> impl Responder {
    let (tx, rx) = unbounded();
    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::GetProcesses,
        answer_channel: Some(tx),
        ..Default::default()
    });

    match result {
        Err(e) => {println!("{}",e); return HttpResponse::InternalServerError().body("")},
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
            let list = Task {
                processes: procstring_as_list(body)
            };

            return HttpResponse::Ok().json(list);
        }
        _ => {println!("1"); return HttpResponse::InternalServerError().body("")},
    };
}

fn procstring_as_list(str: String) -> Vec<HashMap<String, String>> {
    if str == "" {
        return Vec::new();
    }
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
