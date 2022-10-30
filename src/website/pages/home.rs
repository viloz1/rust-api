//! Functions for the home page.

use std::collections::HashMap;

use rocket::fairing::AdHoc;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket_dyn_templates::Template;

use crate::communication::protocols::{none_request, string_to_rtype, From, Request, RequestType};
use crate::website::guards::timer::TimerRequest;
use crate::website::states::ProcessComm;
use rocket_auth::Auth;

/// Form used to recieve instructions for the processes
/// from the buttons. Description will come in the forms:
///     start-<a>: Start process with id a
///     stop-<a>: Stop process with id a
///     restart-<a>: Restart process with id a
///     restartpull-<a>: Restart and pull process with id a
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    description: &'r str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HomeContext<'a> {
    pub header: &'a str,
    pub index: &'a str,
    pub processes: Vec<HashMap<String, String>>,
}

/// Called when the client requests /. The auth guard checks
/// if the user is logged in. If the user is not logged in,
/// redirect the user to the login page. Otherwise,
/// request the processes from the process handler and
/// render the homepage with HomeContext.
#[get("/")]
fn index(auth: Auth<'_>, state: &State<ProcessComm>) -> Result<Template, Redirect> {
    println!("hej");
    if !auth.is_auth() {
        return Err(Redirect::to("/login"));
    }

    let empty = none_request();
    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::GetProcesses,
        ..empty
    });
    match result {
        Err(_) => println!("Could not get processes from handler"),
        _ => (),
    }

    let processes_list = state.receiver.recv().unwrap().processes;

    Ok(Template::render(
        "index",
        &HomeContext {
            header: "Header",
            index: "Website",
            processes: processes_list.unwrap(),
        },
    ))
}

/// Called when the client sends a post request to /, in
/// other words when one of the process buttons are pressed.
///
/// The function first checks if the user is authorized with
/// the auth guard, and returns the Unauthorized http status
/// if not. Otherwise, the processes are fetched again from
/// the process handler and then sends another request to
/// the handler to do the request that was requested by
/// the client.

#[allow(unused_variables)]
#[post("/", format = "application/x-www-form-urlencoded", data = "<task>")]
fn new(
    auth: Auth<'_>,
    task: Json<Task<'_>>,
    state: &State<ProcessComm>,
    time: TimerRequest,
) -> Result<Template, (Status, &'static str)> {
    if !auth.is_auth() {
        return Err((
            Status::Unauthorized,
            "You need to be authorized to access this",
        ));
    }

    let empty = none_request();
    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::GetProcesses,
        ..empty
    });

    match result {
        Err(_) => println!("Could not get processes from handler"),
        _ => (),
    }

    let processes_list = state.receiver.recv().unwrap().processes;
    let request_operation: Vec<&str> = task.description.split("-").collect();

    let empty = none_request();
    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: string_to_rtype(request_operation[0]),
        id: Some(request_operation[1].parse::<usize>().unwrap()),
        ..empty
    });
    match result {
        Err(_) => println!("Could not get processes from handler"),
        _ => (),
    }

    if task.description.is_empty() {
        Ok(Template::render("index", &*task))
    } else {
        match task.description {
            "upvote" => Ok(Template::render(
                "index",
                &HomeContext {
                    header: "Header lmao",
                    index: "You pressed the upvote button",
                    processes: processes_list.unwrap(),
                },
            )),
            _ => Ok(Template::render(
                "index",
                &HomeContext {
                    header: "header lmao",
                    index: &task.description,
                    processes: processes_list.unwrap(),
                },
            )),
        }
    }
}

/// Stage the home page. Used in attach in the main Rocket launch. This is
/// to make sure that Rocket manages the home page.
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Home page", |rocket| async {
        rocket.mount("/", routes![index, new])
    })
}
