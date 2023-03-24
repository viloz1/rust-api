use std::fmt::{Display, self};

use actix_web::cookie::Cookie;
use actix_web::{FromRequest, ResponseError, HttpRequest, web};
use futures::Future;

use super::cookie::{LoginSession, self};
use super::session;
use super::user::User;
use super::users::UserManager;
use tokio::macros::support::Pin;

pub struct Auth{
    pub user: Option<User>
}

#[derive(Debug)]
pub enum AuthError {
    NotAuthenticated
}

impl Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthError::NotAuthenticated => write!(f, "{}", "NotAuthenticated")
        }
    }
}

impl ResponseError for AuthError {}

impl FromRequest for Auth {

    type Error = AuthError;
    type Future = futures_util::future::LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone(); 
        Box::pin(async move {

            let users = req.app_data::<web::Data<UserManager>>().unwrap();

            let auth_cookie: Cookie;
            let session: LoginSession;


            if let Some(auth_cookie_) = req.cookie("viloz-auth") {
                auth_cookie = auth_cookie_;
            } else {
                println!("No cookie found");
                return Err(AuthError::NotAuthenticated);
            }

            if let Ok(session_) = cookie::get_session(auth_cookie) {
                session = session_;
            } else {
                println!("No session matching");
                return Err(AuthError::NotAuthenticated);
            }

            if let Some(user) = users.is_auth(session) {
                return Ok(Auth{user: Some(user)})
            } else {
                return Err(AuthError::NotAuthenticated);
            }

        })
    }
}