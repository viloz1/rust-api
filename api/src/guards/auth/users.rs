use argon2::Error;
use chashmap::CHashMap;
use dashmap::{DashMap, DashSet};
use rand::distributions::{Alphanumeric, Standard, Uniform};
use sqlx::{Pool, Sqlite};
use crate::database::{self, auth::AuthConnection};
use crate::guards::auth::session::SessionManager;
use crate::guards::auth::user::User;

use super::cookie::LoginSession;
use super::session::AuthKey;
use super::user;
use rand::{random, Rng};

#[derive(Clone)]
pub struct UserManager {
    conn: AuthConnection,
    session: SessionManager
}

pub enum LoginError {
    NoUser,
    InternalError
}

pub fn rand_string(size: usize) -> String {

    //Cookies cannnot take values on the form \{<number>}. The first part thus generates only ascii characters.
    //The session also breaks if there is a ; in the cookie, so replace this with a random alphanumeric character

    rand::thread_rng()
        .sample_iter(Uniform::new(char::from(32), char::from(126)))
        .take(size)
        .map(char::from)
        .collect::<String>()
        .replace(";",
        &rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1)
            .map(char::from)
            .collect::<String>()
        )
}

impl UserManager {

    pub async fn connect_db(pool: Pool<Sqlite>) -> UserManager {
        let manager = UserManager {
            conn: AuthConnection::new(pool).await,
            session: SessionManager::new()
        };
        return manager;
    }

    pub async fn create_user(&self, username: String, password: String, role: String) -> Option<User> {
        let mut user = User::new(username, password, role).unwrap();

        if let Ok(id) = self.conn.add_user_to_db(user.clone()).await {
            user.id = id;

            return Some(user)
        } else {return None}

    }

    pub fn modify_user(&self, user: User) {

    }

    fn set_auth_key(&self, user_id: usize) -> LoginSession {
        let secret = rand_string(128);
        self.session.insert(user_id, secret.clone(), 60*15);
        LoginSession { id: user_id, auth_key: secret }
    }

    pub async fn is_auth(&self, session: LoginSession) -> Option<User> {
        if let Some(secret) = self.session.get(session.id) {
            if secret.eq(&session.auth_key) {
                let user = self.conn.get_user_by_id(session.id).await.unwrap();
                return Some(user)
            }
            return None;
        } else {
            None
        }
    }

    pub async fn login(&self, username: String, password: String) -> Result<LoginSession, LoginError> {
        if let Ok(mut user) = self.conn.get_user_by_name(username).await {
            match user.check_password(password) {
                Ok(true) => {
                    return Ok(self.set_auth_key(user.id))
                }
                Ok(false) => {
                    return Err(LoginError::NoUser);
                }
                Err(_) => {
                    return Err(LoginError::InternalError);
                }
            }
        } else {
            return Err(LoginError::NoUser);
        }
    }
    
    pub fn logout(&self, id: usize) {
        self.session.remove(id);

    }

    pub fn clear_expired(&self) {
        self.session.clear_expired();
    }
    
}