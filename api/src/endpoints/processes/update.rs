use actix_web::{post, web, Responder, HttpResponse};

use futures::executor;

use crate::database;
use crate::database::processes::ProcessSQLModel;
use crate::guards::auth::auth::Auth;
use crate::states::DBConnections;
use crate::states::ProcessComm;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProcessUpdateRequest {
    pub name: String,
    pub path: String,
    pub start_cmd: String,
    pub stop_cmd: String,
    pub build_cmd: String,
    pub branch: String,
    pub git_url: String
}

#[post("/update/{id}")]
pub async fn update(content: web::Json<ProcessUpdateRequest>, path: web::Path<usize>, p_db: web::Data<DBConnections>, _auth: Auth) -> impl Responder {
    let id = path.into_inner();
    
    let process_model = ProcessSQLModel {
        name: content.name.to_owned(),
        path: content.path.to_owned(),
        start_cmd: content.start_cmd.to_owned(),
        build_cmd: content.build_cmd.to_owned(),
        stop_cmd: content.stop_cmd.to_owned(),
        branch: content.branch.to_owned(),
        git_url: content.git_url.to_owned()
    };

    let result = executor::block_on(database::processes::update_process_in_db(&p_db.process, process_model, id));

    match result {
        Err(_) => HttpResponse::InternalServerError().body(""),
        _ => HttpResponse::Ok().body(""),
    }

}

