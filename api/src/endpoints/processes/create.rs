use std::collections::HashMap;
use std::hash::Hash;

use crossbeam::channel::unbounded;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize};
use rocket::State;
use futures::executor;

use crate::communication::protocols::{
    From, Request, RequestResult, RequestResultStatus, RequestType,
};
use crate::database;
use crate::database::processes::ProcessSQLModel;
use crate::endpoints::{wait_response, HTTPResponse};
use crate::states::{DBConnections, Timeout};
use crate::states::ProcessComm;
use rocket_auth::User;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProcessCreateRequest<'r> {
    pub name: &'r str,
    pub start_cmd: &'r str,
    pub stop_cmd: &'r str,
    pub build_cmd: &'r str,
    pub branch: &'r str,
    pub git_url: &'r str
}

#[post("/create", data="<content>")]
pub fn create<'a>(content: Json<ProcessCreateRequest<'_>>, auth: User, timeout: &State<Timeout>, state: &'a State<ProcessComm>, p_db: &'a State<DBConnections>) -> Custom<Json<HTTPResponse<'a>>> {
    let process_model = ProcessSQLModel {
        name: content.name.to_string(),
        path: "".to_string(),
        start_cmd: content.start_cmd.to_string(),
        build_cmd: content.build_cmd.to_string(),
        stop_cmd: content.stop_cmd.to_string(),
        branch: content.branch.to_string(),
        git_url: content.git_url.to_string()
    };

    let result = executor::block_on(database::processes::add_process_to_db(&p_db.process, process_model));
    let id: usize;

    match result {
        Err(e) => {error!("Failed to create a process for request /create: {:?}", e); return Custom(Status::InternalServerError, Json(HTTPResponse{content: "Failed to create process"}))},
        Ok(i) => id = i,
    };

    let (tx,rx) = unbounded();

    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::ProcessAdded,
        id: Some(id),
        answer_channel: Some(tx),
        ..Default::default()
    });

    match result {
        Err(e) => {error!("Failed to create a process for request /create: {:?}", e); return Custom(Status::InternalServerError, Json(HTTPResponse{content: "Failed to create process"}))},
        _ => (),
    };

    return wait_response(timeout.timeout, rx);


}

 