use crate::communication::protocols::Request;
use crossbeam::channel::{Receiver, Sender};
use sqlx::Pool;
use sqlx::Sqlite;

pub struct Timeout {
    pub timeout: usize,
}

pub struct DBConnections {
    pub process: Pool<Sqlite>
}

pub struct ProcessComm {
    pub sender: Sender<Request>,
}
