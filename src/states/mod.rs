pub mod processcomm;
use crate::communication::protocols::Request;
use crossbeam::channel::{Receiver, Sender};
use rocket::{fairing::AdHoc, time::Time};

/// Stage the states. Used in attach in the main Rocket launch. This is
/// to make sure that Rocket manages the states.
pub fn stage(tx: Sender<Request>, timeout: usize) -> AdHoc {
    AdHoc::on_ignite("States", move |rocket| async move {
        let proc_com = processcomm::ProcessComm { sender: tx };
        rocket.manage(proc_com).manage(Timeout { timeout })
    })
}

pub struct Timeout {
    pub timeout: usize,
}
