//! Handle requests from github.

use crate::communication::protocols::{none_request, From, Request, RequestType};
use crate::guards::githubip::GithubIP;
use crate::states::processcomm::ProcessComm;
use rocket::serde::{json::Json, Deserialize};
use rocket::{post, State};

/// This struct is used to package the github JSON request into
/// a usable struct. The struct only has one field, ref, since
/// it's the only part about the github request that we care
/// about since it contains the branch name.
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GitHubJSON<'r> {
    r#ref: &'r str, //Will recieve in form "refs/head/<branch>"
}

/// This is a rocket post method that cathes requests made to
/// /github/<id>. The id part is the process id that handles
/// this github project.
///
/// When a github request is received, this functions sends
/// a request to the handler to restart and pull the relevant
/// process.
#[allow(unused_variables)]
#[post("/github/<id>", format = "json", data = "<data>")]
pub async fn github<'a>(
    id: usize,
    data: Json<GitHubJSON<'_>>,
    state: &State<ProcessComm>,
    ip: GithubIP,
) -> &'a str {
    let split: Vec<&str> = data.r#ref.split("/").collect();
    println!("{}", split[split.len() - 1].to_string());
    let empty = none_request();
    let result = state.sender.clone().send(Request {
        from: From::Rocket,
        rtype: RequestType::Github,
        id: Some(id),
        push_branch: Some(split[split.len() - 1].to_string()),
        ..empty
    });

    match result {
        Err(a) => println!(
            "ERROR: Could not send RestartPull request to handler for id {}. Cause: {}",
            id, a
        ),
        _ => (),
    }
    "Approved"
}
