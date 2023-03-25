use std::fmt::{Display, self};

use actix_web::cookie::Cookie;
use actix_web::{FromRequest, ResponseError, HttpRequest, web};

use super::cookie::{LoginSession, self};
use super::user::User;
use super::users::UserManager;

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
            println!("what1");
            let users = req.app_data::<web::Data<UserManager>>().unwrap();
            //users.clear_expired();
            println!("what2");
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

            if let Some(user) = users.is_auth(session).await {
                println!("user is authenticated");
                return Ok(Auth{user: Some(user)})
            } else {
                println!("user is not authenticated");
                return Err(AuthError::NotAuthenticated);
            }

        })
    }
}