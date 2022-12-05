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
use crate::communication::protocols::{none_request, From, Request, RequestType};
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
    mailbox: Receiver<Request>,
    rocket_mailman: Sender<Request>,
    handler_mailman: Sender<Request>,
}

impl ProcessHandler {
    /// Start the process handler that will handle
    /// all the other processess.
    /// tx_main is where we will send information to the main loop, and
    /// rx_main is where we will recieve information to the main loop.

    pub fn new(
        mailbox: Receiver<Request>,
        rocket_mailman: Sender<Request>,
        handler_mailman: Sender<Request>,
    ) -> ProcessHandler {
        ProcessHandler {
            processes: HashMap::new(),
            mailbox,
            rocket_mailman,
            handler_mailman,
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

            let hmail = self.handler_mailman.clone();

            thread::spawn(move || new_process.start_loop(hmail, rx2));
        }

        // The handler loop
        loop {
            // Pattern match messages from the frontend
            let mail = self.mailbox.recv();
            match mail {
                Ok(Request {
                    from: From::Rocket,
                    rtype: RequestType::GetProcesses,
                    id: _,
                    processes: _,
                    push_branch: _,
                    status: _,
                }) => {
                    let result = self.rocket_mailman.clone().send(Request {
                        from: From::Handler,
                        rtype: RequestType::GetProcesses,
                        id: None,
                        processes: Some(self.proclist_as_string()),
                        push_branch: None,
                        status: None,
                    });

                    match result {
                        Err(e) => println!(
                            "ERROR: Could not send back processes to frontend. Cause: {}",
                            e
                        ),
                        _ => (),
                    }
                }

                Ok(Request {
                    from: From::Rocket,
                    rtype: RequestType::Github,
                    id: Some(id),
                    processes: _,
                    push_branch: branch,
                    status: _,
                }) => {
                    let process = self.processes.get(&id).unwrap();
                    if process.branch != branch.unwrap() {
                        {}
                    }
                    //Make sure that process is not None!
                    let empty = none_request();
                    let result = process.sender.send(Request {
                        from: From::Handler,
                        rtype: RequestType::RestartPull,
                        ..empty
                    });

                    match result {
                        Err(e) => println!(
                            "ERROR: Could not tell process {} to restartpull. Cause: {}",
                            id, e
                        ),
                        _ => (),
                    }
                }

                Ok(Request {
                    from: From::Rocket,
                    rtype: RequestType::Start,
                    id: Some(id),
                    processes: _,
                    push_branch: _,
                    status: _,
                }) => {
                    let process = self.processes.get(&id).unwrap();
                    //Make sure that process is not None!
                    let empty = none_request();
                    match process.get_status() {
                        ProcessStatus::Off => {
                            let result = process.sender.send(Request {
                                from: From::Handler,
                                rtype: RequestType::Start,
                                ..empty
                            });
                            match result {
                                Err(e) => println!(
                                    "ERROR: Could not tell process {} to start. Cause: {}",
                                    id, e
                                ),
                                _ => (),
                            }
                        }
                        _ => println!("Error, process is busy"),
                    }
                }

                Ok(Request {
                    from: From::Rocket,
                    rtype: RequestType::Stop,
                    id: Some(id),
                    processes: _,
                    push_branch: _,
                    status: _,
                }) => {
                    let process = self.processes.get(&id).unwrap();
                    //Make sure that process is not None!
                    let empty = none_request();
                    match process.get_status() {
                        ProcessStatus::On => {
                            let result = process.sender.send(Request {
                                from: From::Handler,
                                rtype: RequestType::Stop,
                                ..empty
                            });
                            match result {
                                Err(e) => println!(
                                    "ERROR: Could not tell process {} to stop. Cause: {}",
                                    id, e
                                ),
                                _ => (),
                            }
                        }
                        _ => println!("Error, process is busy"),
                    }
                }

                Ok(Request {
                    from: From::Rocket,
                    rtype: RequestType::Restart,
                    id: Some(id),
                    processes: _,
                    push_branch: _,
                    status: _,
                }) => {
                    let process = self.processes.get(&id).unwrap();
                    //Make sure that process is not None!
                    let empty = none_request();
                    match process.get_status() {
                        ProcessStatus::On => {
                            let result = process.sender.send(Request {
                                from: From::Handler,
                                rtype: RequestType::Restart,
                                ..empty
                            });
                            match result {
                                Err(e) => println!(
                                    "ERROR: Could not tell process {} to restart. Cause: {}",
                                    id, e
                                ),
                                _ => (),
                            }
                        }
                        _ => println!("Error, process is busy"),
                    }
                }

                Ok(Request {
                    from: From::Rocket,
                    rtype: RequestType::RestartPull,
                    id: Some(id),
                    processes: _,
                    push_branch: _,
                    status: _,
                }) => {
                    let process = self.processes.get(&id).unwrap();
                    //Make sure that process is not None!
                    let empty = none_request();
                    let result = process.sender.send(Request {
                        from: From::Handler,
                        rtype: RequestType::RestartPull,
                        ..empty
                    });

                    match result {
                        Err(e) => println!(
                            "ERROR: Could not tell process {} to restartpull. Cause: {}",
                            id, e
                        ),
                        _ => (),
                    }
                }

                Ok(Request {
                    from: From::Process,
                    rtype: RequestType::Status,
                    id: Some(id),
                    processes: _,
                    push_branch: _,
                    status: Some(status),
                }) => {
                    let process = self.processes.get_mut(&id).unwrap();
                    //Make sure that process is not None!

                    process.set_status(status);
                }

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
}
