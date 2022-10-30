//! Handles states used in the website.

use crate::communication::protocols::Request;
use crossbeam::channel::{Receiver, Sender};
use rocket::fairing::AdHoc;

/// This struct enables page functions to extract the sender and receiver
/// to be able to communicate with the process handler. sender is used
/// to send messages to the handler, and receiver is used to receive
/// messages.
pub struct ProcessComm {
    pub sender: Sender<Request>,
    pub receiver: Receiver<Request>,
}

/// Stage the states. Used in attach in the main Rocket launch. This is
/// to make sure that Rocket manages the states.
pub fn stage(tx: Sender<Request>, rx: Receiver<Request>) -> AdHoc {
    AdHoc::on_ignite("States", |rocket| async {
        let proc_com = ProcessComm {
            sender: tx,
            receiver: rx,
        };
        rocket.manage(proc_com)
    })
}
