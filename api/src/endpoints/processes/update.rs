use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize};
use rocket::State;
use futures::executor;

use crate::database;
use crate::database::processes::ProcessSQLModel;
use crate::endpoints::HTTPResponse;
use crate::states::DBConnections;
use crate::states::ProcessComm;
use rocket_auth::User;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProcessCreateRequest<'r> {
    pub name: &'r str,
    pub path: &'r str,
    pub start_cmd: &'r str,
    pub stop_cmd: &'r str,
    pub build_cmd: &'r str,
    pub branch: &'r str,
    pub git_url: &'r str
}

#[post("/update/<id>", data="<content>")]
pub fn update<'a>(id: usize, content: Json<ProcessCreateRequest<'_>>, auth: User, state: &'a State<ProcessComm>, p_db: &'a State<DBConnections>) -> Custom<Json<HTTPResponse<'a>>> {
    println!("{}", content.name);
    
    let process_model = ProcessSQLModel {
        name: content.name.to_string(),
        path: content.path.to_string(),
        start_cmd: content.start_cmd.to_string(),
        build_cmd: content.build_cmd.to_string(),
        stop_cmd: content.stop_cmd.to_string(),
        branch: content.branch.to_string(),
        git_url: content.git_url.to_string()
    };

    let result = executor::block_on(database::processes::update_process_in_db(&p_db.process, process_model, id));

    match result {
        Err(_) => Custom(Status::InternalServerError, Json(HTTPResponse { content: "Internal errer" })),
        _ => Custom(Status::Ok, Json(HTTPResponse { content: "Success" })),
    }

}

