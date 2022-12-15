//! Protocols used in the communcation

use crate::process_handler::process::ProcessStatus;
use crossbeam::channel::Sender;
use std::collections::HashMap;
use std::fmt;

/// Enum to specify the request from the frontend
#[derive(Clone, Debug)]
pub enum From {
    Rocket,
    Process,
    Handler,
}

#[derive(Clone, Debug)]
pub enum RequestType {
    Status,
    Start,
    Restart,
    Stop,
    RestartPull,
    GetProcesses,
    Github,
    ProcessAdded,
    ProcessUpdated,
    ProcessRemoved
}

impl fmt::Display for RequestType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestType::Status => write!(f, "Status"),
            RequestType::Start => write!(f, "Start"),
            RequestType::Restart => write!(f, "Restart"),
            RequestType::Stop => write!(f, "Stop"),
            RequestType::RestartPull => write!(f, "RestartPull"),
            RequestType::GetProcesses => write!(f, "GetProcesses"),
            RequestType::Github => write!(f, "Github"),
            RequestType::ProcessAdded => write!(f, "ProcessAdded"),
            RequestType::ProcessRemoved => write!(f, "ProcessRemoved"),
            RequestType::ProcessUpdated => write!(f, "ProcessUpdated"),
        }
    }
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
#[derive(Clone, Debug)]
pub struct Request {
    pub from: From,
    pub rtype: RequestType,
    pub id: Option<usize>,
    pub processes: Option<Vec<HashMap<String, String>>>,
    pub push_branch: Option<String>,
    pub status: Option<ProcessStatus>,
    pub answer_channel: Option<Sender<RequestResult>>,
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
            answer_channel: None,
        }
    }
}

#[derive(Debug)]

pub enum RequestResultStatus {
    Success,
    Failed,
    Update,
}
#[derive(Debug)]
pub struct RequestResult {
    pub status: RequestResultStatus,
    pub body: Option<String>,
    pub process_status: Option<ProcessStatus>,
    pub id: Option<usize>,
}

impl Default for RequestResult {
    fn default() -> Self {
        RequestResult {
            status: RequestResultStatus::Update,
            body: None,
            process_status: None,
            id: None,
        }
    }
}
