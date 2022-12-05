pub mod processcomm;
use crate::communication::protocols::Request;
use crossbeam::channel::{Receiver, Sender};
use rocket::{fairing::AdHoc, time::Time};

/// Stage the states. Used in attach in the main Rocket launch. This is
/// to make sure that Rocket manages the states.
pub fn stage(tx: Sender<Request>, rx: Receiver<Request>, timeout: usize) -> AdHoc {
    AdHoc::on_ignite("States", |rocket| async {
        let proc_com = processcomm::ProcessComm {
            sender: tx,
            receiver: rx,
        };
        rocket.manage(proc_com);
        rocket.manage(Timeout{timeout})

    })
}

pub struct Timeout {
    pub timeout: usize
}