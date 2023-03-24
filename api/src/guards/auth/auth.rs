use actix_web::{FromRequest, ResponseError};
use futures::Future;

use super::user::User;
use tokio::macros::support::Pin;

pub struct Auth{}

#[derive(Debug)]
pub enum AuthError {
    WrongIP,
}

impl ResponseError for AuthError {}

impl FromRequest for Auth {
    type Error = AuthError;
    type Future = futures_util::future::LocalBoxFuture<'static, Result<Self, Self::Error>>;
}