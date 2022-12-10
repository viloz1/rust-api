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
use crate::states::DBConnections;
use crate::states::ProcessComm;
use rocket_auth::User;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProcessCreateRequest<'r> {
    pub name: &'r str,
    pub path: &'r str,
    pub start_path: &'r str,
    pub stop_path: &'r str,
    pub build_path: &'r str,
    pub branch: &'r str,
    pub git_url: &'r str
}

#[post("/create", data="<content>")]
pub fn create<'a>(content: Json<ProcessCreateRequest<'_>>, auth: User, state: &'a State<ProcessComm>, p_db: &'a State<DBConnections>) -> Custom<&'a str> {
    let process_model = ProcessSQLModel {
        name: content.name.to_string(),
        path: content.path.to_string(),
        start_path: content.start_path.to_string(),
        build_path: content.build_path.to_string(),
        stop_path: content.stop_path.to_string(),
        branch: content.branch.to_string(),
        git_url: content.git_url.to_string()
    };

    let result = executor::block_on(database::processes::add_process_to_db(&p_db.process, process_model));

    match result {
        Err(_) => Custom(Status::InternalServerError, "Failed to create a process"),
        _ => Custom(Status::Ok, "Successfully created a new process"),
    }

}

