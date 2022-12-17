//! Contains the process struct and functions
//! to start a given process.

use crate::communication::protocols::{
    From, Request, RequestResult, RequestResultStatus, RequestType,
};
use crossbeam::channel::{Receiver, Sender};
use run_script;
use run_script::types::ScriptOptions;
use serde::Serialize;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Enum to differentiate the status of the procces
#[derive(Clone, PartialEq, Debug, Serialize)]
pub enum ProcessStatus {
    On,
    Starting,
    Building,
    Pulling,
    Terminating,
    Off,
}

/// Convert ProcessStatus to a String
pub fn status_to_string(status: ProcessStatus) -> String {
    match status {
        ProcessStatus::On => "On".to_string(),
        ProcessStatus::Starting => "Starting".to_string(),
        ProcessStatus::Pulling => "Pulling".to_string(),
        ProcessStatus::Terminating => "Terminating".to_string(),
        ProcessStatus::Off => "Off".to_string(),
        ProcessStatus::Building => "Building".to_string()
    }
}

/// The process struct and it's fields
#[derive(Clone, Debug)]
pub struct Process {
    pub path: String,
    pub git_url: String,
    pub status: ProcessStatus,
    pub process_id: usize,
    pub name: String,
    pub sender: Sender<Request>,
    pub branch: String,
    pub start_cmd: String,
    pub stop_cmd: String,
    pub build_cmd: String
}

impl Process {
    /// Determines the channel the procces should use
    /// to communicate with the handler
    pub fn set_sender(&mut self, new_sender: Sender<Request>) {
        self.sender = new_sender;
    }

    /// Get the name of the process
    pub fn get_name(&mut self) -> String {
        self.name.clone()
    }

    /// Set the status of the procces
    pub fn set_status(&mut self, new_status: ProcessStatus) {
        self.status = new_status;
    }

    /// Get the status of the process
    pub fn get_status(&self) -> ProcessStatus {
        self.status.clone()
    }

    /// Get the id of the process
    pub fn get_id(&self) -> usize {
        self.process_id.clone()
    }

    pub fn is_busy(&self) -> bool {
        return self.status == ProcessStatus::Terminating
            || self.status == ProcessStatus::Starting
            || self.status == ProcessStatus::Pulling
            || self.status == ProcessStatus::Building;
    }

    /// Start a procces. The programme will search for a build script
    /// and a start script in <process_directory>/api. Build and start
    /// scripts needs to be .sh files.
    pub fn start(&mut self, tx: Sender<RequestResult>) -> anyhow::Result<std::process::Child> {
        self.set_status(ProcessStatus::Building);

        tx.send(RequestResult {
            status: RequestResultStatus::Update,
            process_status: Some(ProcessStatus::Building),
            id: Some(self.get_id()),
            ..Default::default()
        })?;

        Command::new("make")
                .arg(self.build_cmd.clone())
                .current_dir(self.path.clone())
                .output()?;

        self.set_status(ProcessStatus::Starting);

        tx.send(RequestResult {
            status: RequestResultStatus::Update,
            process_status: Some(ProcessStatus::Starting),
            id: Some(self.get_id()),
            ..Default::default()
        })?;

        let child = Command::new("make")
                .arg(self.start_cmd.clone())
                .current_dir(self.path.clone())
                .stdout(Stdio::null())
                .spawn()?;


        self.set_status(ProcessStatus::On);

        tx.send(RequestResult {
            status: RequestResultStatus::Update,
            process_status: Some(ProcessStatus::On),
            id: Some(self.get_id()),
            ..Default::default()
        })?;

        Ok(child)
    }

    /// Stop a procces. The programme will search for a stop
    /// script in <process_directory>/api. Stop
    /// script need to be a .sh file.
    pub fn stop(&mut self, tx: Sender<RequestResult>) -> anyhow::Result<()> {
        self.set_status(ProcessStatus::Terminating);

        tx.send(RequestResult {
            status: RequestResultStatus::Update,
            process_status: Some(ProcessStatus::Terminating),
            id: Some(self.get_id()),
            ..Default::default()
        })?;

        Command::new("make")
                .arg(self.stop_cmd.clone())
                .current_dir(self.path.clone())
                .output()?;

        self.set_status(ProcessStatus::Off);
        
        tx.send(RequestResult {
            status: RequestResultStatus::Update,
            process_status: Some(ProcessStatus::Off),
            id: Some(self.get_id()),
            ..Default::default()
        })?;

        Ok(())
    }

    /// Pull a project from github. The programme will search for a pull
    /// script in <process_directory>/api. Pull
    /// script need to be a .sh file.
    pub fn pull(&mut self, tx: Sender<RequestResult>) -> anyhow::Result<()> {
        tx.send(RequestResult {
            status: RequestResultStatus::Update,
            process_status: Some(ProcessStatus::Pulling),
            id: Some(self.get_id()),
            ..Default::default()
        })?;

        Command::new("git")
                .arg("pull")
                .current_dir(self.path.clone())
                .output()?;
        
        tx.send(RequestResult {
            status: RequestResultStatus::Update,
            process_status: Some(ProcessStatus::Off),
            id: Some(self.get_id()),
            ..Default::default()
        })?;

        Ok(())

    }

    /// Main loop for a process. Tries to pattern match requests from
    /// the handler on rx channel, and answers on tx channel.
    pub fn start_loop(&mut self, tx: Sender<RequestResult>, rx: Receiver<Request>) {
        let mut restartandpull: bool = false;
        println!("Loop started for {}", self.get_name());
        //The program won't start if the child spawned isn't assigned (owned)
        //Otherwise, the child is dropped and the process stops
        self.pull(tx.clone());
        let mut child = self.start(tx.clone());
        loop {
            let mail = rx.recv();
            match mail {
                Ok(Request {
                    from: From::Handler,
                    rtype: RequestType::Start,
                    id: _,
                    processes: _,
                    push_branch: _,
                    status: _,
                    answer_channel: _,
                }) => {
                    child = self.start(tx.clone());
                }

                Ok(Request {
                    from: From::Handler,
                    rtype: RequestType::RestartPull,
                    id: _,
                    processes: _,
                    push_branch: _,
                    status: _,
                    answer_channel: _,
                }) => match self.get_status() {
                    ProcessStatus::Off => {
                        self.pull(tx.clone());
                    }
                    ProcessStatus::On => {
                        self.stop(tx.clone());
                        self.pull(tx.clone());
                        child = self.start(tx.clone());
                    }
                    _ => {
                        restartandpull = true;
                    }
                },

                Ok(Request {
                    from: From::Handler,
                    rtype: RequestType::Stop,
                    id: _,
                    processes: _,
                    push_branch: _,
                    status: _,
                    answer_channel: _,
                }) => {
                    self.stop(tx.clone());
                }

                Ok(Request {
                    from: From::Handler,
                    rtype: RequestType::Restart,
                    id: _,
                    processes: _,
                    push_branch: _,
                    status: _,
                    answer_channel: _,
                }) => {
                    self.stop(tx.clone());
                    child = self.start(tx.clone());
                }

                _ => (),
            }
            if restartandpull {
                match self.get_status() {
                    ProcessStatus::Off => {
                        self.pull(tx.clone());
                        restartandpull = false;
                    }
                    ProcessStatus::On => {
                        self.stop(tx.clone());
                        self.pull(tx.clone());
                        self.start(tx.clone());
                        restartandpull = false;
                    }
                    _ => (),
                }
            }
        }
    }
}
