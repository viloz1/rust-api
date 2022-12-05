//! Contains the process struct and functions
//! to start a given process.

use crossbeam::channel::{Sender, Receiver};
use crate::communication::protocols::{Request, RequestType, From, none_request, RequestResult};
use std::path::PathBuf;
use run_script;
use run_script::types::ScriptOptions;
use std::path::PathBuf;

/// Enum to differentiate the status of the procces
#[derive(Clone, PartialEq, Debug, Serialize)]
pub enum ProcessStatus {
    On,
    Starting,
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

    /// Start a procces. The programme will search for a build script
    /// and a start script in <process_directory>/api. Build and start
    /// scripts needs to be .sh files.
    pub fn start(&mut self, tx: Sender<RequestResult>) -> std::process::Child {
        self.set_status(ProcessStatus::Starting);

        let empty = none_request();

        let result = tx.send(Request {
            from: From::Process,
            rtype: RequestType::Status,
            id: Some(self.get_id()),
            status: Some(ProcessStatus::Starting),
            ..empty
        });

        match result {
            Err(e) => println!("ERROR: Process with id {} could not tell process handler that it was starting. Cause: {}",self.get_id(),e),
            _ => ()
        }

        let orig_options = ScriptOptions::new();

        let mut path = PathBuf::new();
        path.push(&self.path);

        let options = ScriptOptions {
            working_directory: Some(path),
            ..orig_options
        };

        let args = vec![];

        let mut run_path: String = self.path.to_string();
        run_path.push_str(r"/api/build.sh");

        run_script::run(run_path.as_str(), &args, &options).unwrap();

        run_path = self.path.to_string();
        run_path.push_str(r"/api/start.sh");
        let child = run_script::spawn(run_path.as_str(), &args, &options).unwrap();

        self.set_status(ProcessStatus::On);
        let empty = none_request();
        let result = tx.send(Request {
            from: From::Process,
            rtype: RequestType::Status,
            id: Some(self.get_id()),
            status: Some(ProcessStatus::On),
            ..empty
        });

        match result {
            Err(e) => println!("ERROR: Process with id {} could not tell process handler that it had started. Cause: {}",self.get_id(),e),
            _ => ()
        };
        child
    }

    /// Stop a procces. The programme will search for a stop
    /// script in <process_directory>/api. Stop
    /// script need to be a .sh file.
    pub fn stop(&mut self, tx: Sender<RequestResult>) -> () {
        self.set_status(ProcessStatus::Terminating);

        let empty = none_request();
        let result = tx.send(Request {
            from: From::Process,
            rtype: RequestType::Status,
            id: Some(self.get_id()),
            status: Some(ProcessStatus::Terminating),
            ..empty
        });

        match result {
            Err(e) => println!("ERROR: Process with id {} could not tell process handler that it was terminating. Cause: {}",self.get_id(),e),
            _ => ()
        }

        let mut stop_path: String = self.path.to_string();
        stop_path.push_str(r"/api/stop.sh");
        run_script::run_script!(stop_path.as_str()).unwrap();

        self.set_status(ProcessStatus::Off);

        let empty = none_request();
        let result = tx.send(Request {
            from: From::Process,
            rtype: RequestType::Status,
            id: Some(self.get_id()),
            status: Some(ProcessStatus::Off),
            ..empty
        });

        match result {
            Err(e) => println!("ERROR: Process with id {} could not tell process handler that it had stopped. Cause: {}",self.get_id(),e),
            _ => ()
        }
    }

    /// Pull a project from github. The programme will search for a pull
    /// script in <process_directory>/api. Pull
    /// script need to be a .sh file.
    pub fn pull(&mut self, tx: Sender<RequestResult>) -> () {
        self.set_status(ProcessStatus::Pulling);

        let empty = none_request();
        let result = tx.send(Request {
            from: From::Process,
            rtype: RequestType::Status,
            id: Some(self.get_id()),
            status: Some(ProcessStatus::Pulling),
            ..empty
        });

        match result {
            Err(e) => println!("ERROR: Process with id {} could not tell process handler that it was pulling. Cause: {}",self.get_id(),e),
            _ => ()
        }

        let orig_options = ScriptOptions::new();

        let mut path = PathBuf::new();
        path.push(&self.path);

        let options = ScriptOptions {
            working_directory: Some(path),
            ..orig_options
        };

        let args = vec![];

        let mut run_path: String = self.path.to_string();
        run_path.push_str(r"/api/pull.sh");

        run_script::run(run_path.as_str(), &args, &options).unwrap();

        self.set_status(ProcessStatus::Off);
        let empty = none_request();
        let result = tx.send(Request {
            from: From::Process,
            rtype: RequestType::Status,
            id: Some(self.get_id()),
            status: Some(ProcessStatus::Off),
            ..empty
        });

        match result {
            Err(e) => println!("ERROR: Process with id {} could not tell process handler that it was off after pulling. Cause: {}",self.get_id(),e),
            _ => ()
        }
    }

    /// Main loop for a process. Tries to pattern match requests from
    /// the handler on rx channel, and answers on tx channel.
    pub fn start_loop(&mut self, tx: Sender<RequestResult>, rx: Receiver<Request>) {
        let mut restartandpull: bool = false; 
        println!("Loop started for {}", self.get_name());
        //The program won't start if the child spawned isn't assigned (owned)
        //Otherwise, the child is dropped and the process stops
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
