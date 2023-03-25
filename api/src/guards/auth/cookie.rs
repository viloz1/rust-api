use std::{fmt, str::FromStr};

use actix_web::cookie::Cookie;
use log::Log;

pub struct LoginSession {
    pub id: usize,
    pub auth_key: String
}

pub enum LoginSessionError {
    InvalidSession
}

impl LoginSession {
    pub fn to_string(&self) -> String {
        format!("{}-{}", self.id, self.auth_key)
    }
}

impl FromStr for LoginSession {

    type Err = LoginSessionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(index) = s.chars().position(|c| c == '-') {
            let id_str = &s[..index]; 
            let auth_key = &s[index+1..];

            let id_res = id_str.to_string().parse::<usize>();

            match id_res {
                Ok(id) => {
                    Ok(LoginSession {
                        id,
                        auth_key: auth_key.to_string()
                    })
                }
                Err(_) => Err(LoginSessionError::InvalidSession)
            }
        } else {
            Err(LoginSessionError::InvalidSession)
        }
        
    }
}

pub fn get_session(cookie: Cookie) -> Result<LoginSession, LoginSessionError> {
    let session: LoginSession = LoginSession::from_str(cookie.value())?;
    Ok(session)
}