//! The process_handler manages all the smaller processes,
//! on request from the frontend. Every action meant for
//! a process goes through this handler first, to make
//! sure that the request is valid before sending
//! it to the processes. This handler can communicate
//! with the website by using message passing.

use crossbeam::channel::{unbounded, Receiver, RecvError, Select, Sender};
use futures::executor;
use rocket_auth::Result;
use run_script::ScriptOptions;
use serde::__private::de;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;
use std::thread;
pub mod process;
use crate::communication::protocols::RequestResult;
use crate::database;
use crate::communication::protocols::RequestResultStatus;
use crate::communication::protocols::{From, Request, RequestType};
use crate::database::processes::ProcessSQLModel;
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

pub struct ProcessHandler<'r> {
    processes: HashMap<usize, Process>,
    rocket_handler_ch: Receiver<Request>,
    process_handler_ch: Receiver<RequestResult>,
    handler_process_ch: Sender<RequestResult>,
    process_pool: &'r SqlitePool
}

impl ProcessHandler<'_> {
    /// Start the process handler that will handle
    /// all the other processess.
    /// tx_main is where we will send information to the main loop, and
    /// rx_main is where we will recieve information to the main loop.

    pub fn new(rocket_handler_ch: Receiver<Request>, pool: &SqlitePool) -> ProcessHandler {
        let (handler_process_ch, process_handler_ch) = unbounded();
        ProcessHandler {
            processes: HashMap::new(),
            rocket_handler_ch,
            process_handler_ch,
            handler_process_ch,
            process_pool: pool
        }
    }

    pub async fn start_process(&mut self, name: String, process_id: usize, git_url: String, branch: String, mut path: String, start_cmd: String, stop_cmd: String, build_cmd: String) {
        let (tx2, rx2) = unbounded();
        println!("Starting process");
        let mut path_buf = PathBuf::from("projects/");
        path_buf.push(name.clone());

        if path == "" || !Path::new(&path_buf).exists() {

            std::fs::create_dir_all(path_buf.clone()).expect("failed to create dir");

            println!("No path found, pulling");

            let cmd = Command::new("git")
                .arg("clone")
                .arg("-b")
                .arg(branch.clone())
                .arg(git_url.clone())
                .arg(".")
                .current_dir(path_buf.clone())
                .output();
            
            match cmd {
                Err(e) => {println!("Failed: {:?}",e); return},
                Ok(a) => println!("Success: {:?}",a.stdout)
            };

            path = path_buf.as_os_str().to_str().unwrap().to_string();

            let updated_process = ProcessSQLModel {
                name: name.clone(),
                path: path.clone(),
                start_cmd: start_cmd.clone(),
                stop_cmd: stop_cmd.clone(),
                build_cmd: build_cmd.clone(),
                branch: branch.clone(),
                git_url: git_url.clone()
            };

            database::processes::update_process_in_db(self.process_pool, updated_process, process_id).await;
        }

        let mut process = Process {
            name,
            git_url,
            process_id,
            branch,
            path,
            start_cmd,
            stop_cmd,
            build_cmd,
            status: ProcessStatus::Off,
            sender: tx2
        };

        println!("Started new process");

        let hmail = self.handler_process_ch.clone();
        self.processes.insert(process.get_id(), process.clone());
        println!("{:?}",self.processes);
        thread::spawn(move || process.start_loop(hmail, rx2));
    }

    #[tokio::main]
    pub async fn start(&mut self, pool: &SqlitePool) {
        // Get the stored processes, and spawn a new thread for every
        // one
        let result = database::processes::get_all_proccesses(pool).await;
        let db_processes: Vec<(usize, ProcessSQLModel)>;
        match result {
            Err(e) => {println!("exiting"); std::process::exit(1)},
            Ok(r) => db_processes = r
        }

        println!("{:?}", db_processes);

        for (id, proc) in db_processes {
            self.start_process(
                proc.name,
                id,
                proc.git_url,
                proc.branch,
                proc.path,
                proc.start_cmd,
                proc.stop_cmd,
                proc.build_cmd
            ).await;
        }

        let mut sel = Select::new();
        let rocket_handler_ch_clone = self.rocket_handler_ch.clone();
        let process_handler_ch_clone = self.process_handler_ch.clone();
        sel.recv(&rocket_handler_ch_clone);
        sel.recv(&process_handler_ch_clone);
        loop {
            let oper = sel.ready();
            if oper == 0 {
                self.handle_api_requests(rocket_handler_ch_clone.recv());
            } else {
                self.handle_process_requests(process_handler_ch_clone.recv());
            }
        }
    }

    fn handle_process_requests(&mut self, mail: Result<RequestResult, RecvError>) {
        match mail {
            Ok(RequestResult {
                status: RequestResultStatus::Update,
                body: body,
                id: Some(id),
                process_status: Some(process_status),
            }) => {
                let process = self.processes.get_mut(&id).unwrap();
                println!("Recieved update: {:?}", process_status);

                process.set_status(process_status);
            }

            a => println!("Recieved unspecified message: {:?}", a),
        }
    }

    fn handle_api_requests(&mut self, mail: Result<Request, RecvError>) {
        // Pattern match messages from the frontend
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
                let body = proclist_as_string(self.processes.clone());
                let answer = RequestResult {
                    status: RequestResultStatus::Success,
                    body: Some(body),
                    ..Default::default()
                };
                answer_channel.send(answer);
            }

            Ok(Request {
                from: From::Rocket,
                rtype: RequestType::ProcessAdded,
                id: Some(id),
                processes: _,
                push_branch: _,
                status: _,
                answer_channel: Some(answer_channel),
            }) => {
                let result = executor::block_on(database::processes::get_process_by_id(id, self.process_pool));
                let proc: (usize, ProcessSQLModel);
                match result {
                    Err(_) => {send_reply(RequestResultStatus::Failed, answer_channel, None); return}
                    Ok(p) => proc = p
                };

                self.start_process(
                    proc.1.name,
                    proc.0,
                    proc.1.git_url,
                    proc.1.branch,
                    proc.1.path,
                    proc.1.start_cmd,
                    proc.1.stop_cmd,
                    proc.1.build_cmd
                );
                send_reply(RequestResultStatus::Success, answer_channel, None);
            }
            /* 
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
            */

            Ok(Request {
                from: From::Rocket,
                rtype: RequestType::Github,
                id: Some(id),
                processes: _,
                push_branch: Some(branch),
                status: _,
                answer_channel: Some(answer_channel),
            }) => {
                let process = self.processes.get_mut(&id).unwrap();
                if process.branch != branch {
                    send_reply(RequestResultStatus::Failed, answer_channel, None);
                    return;
                }

                self.send_action(id, answer_channel, RequestType::RestartPull);
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

            Ok(m) => println!("Recieved unspecified message: {:?}", m),
            _ => println!("Receieved error")
        }
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

fn proclist_as_string(list: HashMap<usize, Process>) -> String {
    if list.len() == 0 {
        return "".to_string()
    }

    let mut rv: String = String::new();

    for (_, mut process) in list {
        rv.push_str(process.get_name().as_str());
        rv.push(',');
        rv.push_str(process.get_id().to_string().as_str());
        rv.push(',');
        rv.push_str(status_to_string(process.get_status()).as_str());
        rv.push(':');
    }
    rv.pop();
    rv
}
