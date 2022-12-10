use crate::communication::protocols::Request;
use crossbeam::channel::{Receiver, Sender};
use rocket::{fairing::AdHoc, time::Time};
use sqlx::Pool;
use sqlx::Sqlite;

/// Stage the states. Used in attach in the main Rocket launch. This is
/// to make sure that Rocket manages the states.
pub fn stage(tx: Sender<Request>, timeout: usize, process: Pool<Sqlite>) -> AdHoc {
    AdHoc::on_ignite("States", move |rocket| async move {
        let proc_com = ProcessComm { sender: tx };
        rocket.manage(proc_com).manage(Timeout { timeout }).manage(DBConnections{process})
    })
}

pub struct Timeout {
    pub timeout: usize,
}

pub struct DBConnections {
    pub process: Pool<Sqlite>
}

pub struct ProcessComm {
    pub sender: Sender<Request>,
}
