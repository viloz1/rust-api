use std::{fmt, str::FromStr};

use actix_web::cookie::Cookie;
use log::Log;

pub struct LoginSession {
    pub id: usize,
    pub username: String,
    pub auth_key: String
}

pub enum LoginSessionError {
    InvalidSession
}

impl LoginSession {
    pub fn to_string(&self) -> String {
        format!("{}-{}-{}", self.id, self.username, self.auth_key)
    }
}

impl FromStr for LoginSession {

    type Err = LoginSessionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split('-').collect::<Vec<&str>>();

        if split.len() != 3 {
            return Err(LoginSessionError::InvalidSession);
        }

        if let Ok(id) = split[0].parse::<usize>() {
            Ok(LoginSession {
                id: id,
                username: split[1].to_string(),
                auth_key: split[2].to_string()
            })
        } else {
            Err(LoginSessionError::InvalidSession)
        }
    }
}

pub fn get_session(cookie: Cookie) -> Result<LoginSession, LoginSessionError> {
    let session: LoginSession = LoginSession::from_str(cookie.value())?;
    Ok(session)
}