//! Protocols used in the communcation

use crate::process_handler::process::{ProcessStatus};
use std::collections::HashMap;

/// Enum to specify the request from the frontend

pub fn none_request() -> Request {
    Request{
        from: From::Handler,
        rtype: RequestType::Start,
        id: None,
        processes: None,
        push_branch: None,
        status: None
    }
} 

pub enum From {
    Rocket,
    Process,
    Handler
}

pub enum RequestType {
    Status,
    Start,
    Restart,
    Stop,
    RestartPull,
    GetProcesses,
    Github
}

pub fn string_to_rtype(string: &str) -> RequestType{
    match string {
        "stop" => RequestType::Stop,
        "start" => RequestType::Start,
        "restart" => RequestType::Restart,
        "restartpull" => RequestType::RestartPull,
        _ => RequestType::RestartPull
    }
}

/// The structure of messages from the frontend
/// to the process handler
pub struct Request {
    pub from: From,
    pub rtype: RequestType,
    pub id: Option<usize>,
    pub processes: Option<Vec<HashMap<String, String>>>,
    pub push_branch: Option<String>,
    pub status: Option<ProcessStatus>,
}
