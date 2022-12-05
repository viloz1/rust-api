//! Protocols used in the communcation

use crate::process_handler::process::ProcessStatus;
use std::collections::HashMap;
use crossbeam::channel::Sender;

/// Enum to specify the request from the frontend

pub enum From {
    Rocket,
    Process,
    Handler,
}

pub enum RequestType {
    Status,
    Start,
    Restart,
    Stop,
    RestartPull,
    GetProcesses,
    Github,
}

pub fn string_to_rtype(string: &str) -> RequestType {
    match string {
        "stop" => RequestType::Stop,
        "start" => RequestType::Start,
        "restart" => RequestType::Restart,
        "restartpull" => RequestType::RestartPull,
        _ => RequestType::RestartPull,
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
    pub answer_channel: Option<Sender<RequestResult>>
}

impl Default for Request {
    fn default() -> Self {
        Request {
            from: From::Handler,
            rtype: RequestType::Stop,
            id: None,
            processes: None,
            push_branch: None,
            status: None,
            answer_channel: None
        }
    }
}

pub enum RequestResultStatus {
    Success,
    Failed,
    Update
}

pub struct RequestResult {
    pub status: RequestResultStatus,
    pub process_status: ProcessStatus,
    pub message_id: usize,
    pub id: usize,
    pub body: String
}