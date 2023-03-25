use crossbeam::channel::unbounded;
use futures::executor;

use actix_web::{post, web, Responder, HttpResponse};

use crate::communication::protocols::{
    From, Request, RequestResult, RequestResultStatus, RequestType,
};
use crate::database;
use crate::database::processes::ProcessSQLModel;
use crate::guards::auth::auth::Auth;
use crate::states::{DBConnections, Timeout};
use crate::states::ProcessComm;
use serde::Deserialize;
use log::error;

#[derive(Deserialize)]
pub struct ProcessCreateRequest {
    pub name: String,
    pub start_cmd: String,
    pub stop_cmd: String,
    pub build_cmd: String,
    pub branch: String,
    pub git_url: String
}

#[post("/create")]
pub async fn create(_auth: Auth, content: web::Json<ProcessCreateRequest>, state: web::Data<ProcessComm>, p_db: web::Data<DBConnections>) -> impl Responder {
    let process_model = ProcessSQLModel {
        name: content.name.to_owned(),
        path: "".to_string(),
        start_cmd: content.start_cmd.to_owned(),
        build_cmd: content.build_cmd.to_owned(),
        stop_cmd: content.stop_cmd.to_owned(),
        branch: content.branch.to_owned(),
        git_url: content.git_url.to_owned()
    };

    let result = executor::block_on(database::processes::add_process_to_db(&p_db.process, process_model));
    let id: usize;

    match result {
        Err(e) => {error!("Failed to create a process for request /create: {:?}", e); return HttpResponse::InternalServerError().body("")},
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
        Err(e) => {error!("Failed to create a process for request /create: {:?}", e); HttpResponse::InternalServerError().body("")},
        _ => HttpResponse::Ok().body(""),
    }



}

 