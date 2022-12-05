//! The process_handler manages all the smaller processes,
//! on request from the frontend. Every action meant for
//! a process goes through this handler first, to make
//! sure that the request is valid before sending
//! it to the processes. This handler can communicate
//! with the website by using message passing.

use crossbeam::channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::fs;
use std::thread;
pub mod process;
use crate::communication::protocols::RequestResult;
use crate::communication::protocols::RequestResultStatus;
use crate::communication::protocols::{From, Request, RequestType};
use process::status_to_string;
use process::{Process, ProcessStatus};
use serde_derive::Deserialize;
use toml;

#[derive(Deserialize)]
struct Config {
    process: Vec<ConfigProcess>,
}

#[derive(Deserialize)]
struct ConfigProcess {
    path: String,
    git_url: String,
    name: String,
    branch: String,
    id: usize,
}

pub struct ProcessHandler {
    processes: HashMap<usize, Process>,
    mailbox_rocket: Receiver<Request>,
    rocket_mailman: Sender<Request>,
    handler_mailman: Sender<Request>,
    mailbox_from_process: Receiver<RequestResult>,
    mailbox_to_process: Sender<RequestResult>,
}

impl ProcessHandler {
    /// Start the process handler that will handle
    /// all the other processess.
    /// tx_main is where we will send information to the main loop, and
    /// rx_main is where we will recieve information to the main loop.

    pub fn new(
        mailbox_rocket: Receiver<Request>,
        rocket_mailman: Sender<Request>,
        handler_mailman: Sender<Request>,
    ) -> ProcessHandler {
        let (mailbox_to_process, mailbox_from_process) = unbounded();
        ProcessHandler {
            processes: HashMap::new(),
            mailbox_rocket,
            rocket_mailman,
            handler_mailman,
            mailbox_from_process,
            mailbox_to_process,
        }
    }

    pub fn start(&mut self) {
        // Get the stored processes, and spawn a new thread for every
        // one
        let map = self.retrieve_processes();
        self.processes = map;

        for (_, value) in &mut self.processes {
            //The channel the processes will listen to
            let (tx2, rx2) = unbounded();

            value.set_sender(tx2);
            let mut new_process = value.clone();
            println!("Started new handler");

            let hmail = self.mailbox_to_process.clone();

            thread::spawn(move || new_process.start_loop(hmail, rx2));
        }

        crossbeam_utils::thread::scope(|s| {
            s.spawn(|_| self.handle_api_requests());
        })
        .unwrap();

        crossbeam_utils::thread::scope(|s| {
            s.spawn(|_| self.handle_process_requests());
        })
        .unwrap();

        loop {}
    }

    fn handle_process_requests(&mut self) {
        loop {
            let mail = self.mailbox_from_process.recv();

            match mail {
                Ok(RequestResult {
                    status: RequestResultStatus::Update,
                    body: body,
                    id: Some(id),
                    process_status: Some(process_status),
                }) => {
                    let process = self.processes.get_mut(&id).unwrap();
                    //Make sure that process is not None!

                    process.set_status(process_status);
                }

                _ => println!("Recieved unspecified message"),
            }
        }
    }

    fn handle_api_requests(&mut self) {
        loop {
            // Pattern match messages from the frontend
            let mail = self.mailbox_rocket.recv();
            match mail {
                Ok(Request {
                    from: From::Rocket,
                    rtype: RequestType::GetProcesses,
                    id: _,
                    processes: _,
                    push_branch: _,
                    status: _,
                    answer_channel: Some(answer_channel),
                }) => {
                    let (tx, rx) = unbounded();
                    let result = self.rocket_mailman.clone().send(Request {
                        from: From::Handler,
                        rtype: RequestType::GetProcesses,
                        id: None,
                        processes: Some(self.proclist_as_string()),
                        push_branch: None,
                        status: None,
                        answer_channel: Some(tx),
                    });
                    match result {
                        Err(e) => {
                            println!(
                                "ERROR: Could not send back processes to frontend. Cause: {}",
                                e
                            );
                            send_reply(RequestResultStatus::Failed, answer_channel, None);
                            continue;
                        }
                        _ => (),
                    }

                    match rx.recv() {
                        Err(e) => {
                            println!(
                                "ERROR: Could not send back processes to frontend. Cause: {}",
                                e
                            );
                            send_reply(RequestResultStatus::Failed, answer_channel, None);
                            continue;
                        }
                        Ok(res) => answer_channel.send(res),
                    };
                }

                Ok(Request {
                    from: From::Process,
                    rtype: RequestType::Status,
                    id: Some(id),
                    processes: _,
                    push_branch: _,
                    status: Some(status),
                    answer_channel: Some(answer_channel),
                }) => {
                    let process = self.processes.get_mut(&id).unwrap();
                    //Make sure that process is not None!

                    process.set_status(status);
                    send_reply(RequestResultStatus::Success, answer_channel, None);
                }

                Ok(Request {
                    from: From::Rocket,
                    rtype: action,
                    id: Some(id),
                    processes: _,
                    push_branch: _,
                    status: _,
                    answer_channel: Some(answer_channel),
                }) => self.send_action(id, answer_channel, action),

                _ => println!("Recieved unspecified message"),
            }
        }
    }

    /// Retrieve the stored processes, and put them
    /// in a vector
    fn retrieve_processes(&mut self) -> HashMap<usize, Process> {
        let file = fs::read_to_string("apiconfig.toml").unwrap();
        let config: Config = toml::from_str(file.as_str()).unwrap();
        let mut map = HashMap::new();
        for process in config.process {
            let (tx1, _) = unbounded::<Request>();
            map.insert(
                process.id,
                Process {
                    path: process.path,
                    git_url: process.git_url,
                    status: ProcessStatus::Off,
                    process_id: process.id,
                    name: process.name,
                    sender: tx1,
                    branch: process.branch,
                },
            );
        }
        println!("{:?}", map);
        return map;
    }

    fn proclist_as_string(&mut self) -> Vec<HashMap<String, String>> {
        let mut rv: Vec<HashMap<String, String>> = vec![];

        for (_, process) in &mut self.processes {
            let mut new_hash: HashMap<String, String> = HashMap::new();
            new_hash.insert("name".to_string(), process.get_name());
            new_hash.insert("id".to_string(), process.get_id().to_string());
            new_hash.insert("status".to_string(), status_to_string(process.get_status()));
            rv.push(new_hash);
        }
        rv
    }

    fn send_action(
        &mut self,
        id: usize,
        answer_channel: Sender<RequestResult>,
        action: RequestType,
    ) {
        let process = self.processes.get(&id).unwrap();
        if process.is_busy() {
            send_reply(
                RequestResultStatus::Failed,
                answer_channel,
                Some("That process is busy with. Please try again later.".to_string()),
            );
            return;
        }
        //Make sure that process is not None!
        let result = process.sender.send(Request {
            from: From::Handler,
            rtype: action.clone(),
            answer_channel: None,
            ..Default::default()
        });

        match result {
            Err(e) => {
                println!(
                    "ERROR: Could not tell process {} to do action {}. Cause: {}",
                    id, action, e
                );
                send_reply(RequestResultStatus::Failed, answer_channel, None);
                return;
            }
            _ => (),
        }
        send_reply(RequestResultStatus::Success, answer_channel, None);
    }
}

fn send_reply(status: RequestResultStatus, sender: Sender<RequestResult>, body: Option<String>) {
    let reply = RequestResult {
        status: status,
        body: body,
        ..Default::default()
    };
    sender.send(reply);
}
