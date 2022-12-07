//! Handles states used in the website.

use crate::communication::protocols::Request;
use crossbeam::channel::{Receiver, Sender};

/// This struct enables page functions to extract the sender and receiver
/// to be able to communicate with the process handler. sender is used
/// to send messages to the handler, and receiver is used to receive
/// messages.
pub struct ProcessComm {
    pub sender: Sender<Request>,
}
