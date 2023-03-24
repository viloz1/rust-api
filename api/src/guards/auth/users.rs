use argon2::Error;
use chashmap::CHashMap;
use sqlx::{Pool, Sqlite};
use crate::database;
use crate::guards::auth::session::SessionManager;
use crate::guards::auth::user::User;

use super::cookie::LoginSession;
use super::session::AuthKey;
use super::user;
use rand::random;

#[derive(Clone)]
pub struct UserManager {
    conn: Pool<Sqlite>,
    session: SessionManager,
    users: CHashMap<String, User>,
    id_map: CHashMap<usize, String>
}

pub enum LoginError {
    NoUser,
    InternalError
}

pub fn rand_string(size: usize) -> String {
    (0..)
        .map(|_| random::<char>())
        .filter(|c| c.is_ascii())
        .map(char::from)
        .take(size)
        .collect()
}

impl UserManager {

    pub async fn connect_db(pool: Pool<Sqlite>) -> UserManager {
        let mut manager = UserManager {
            conn: pool,
            session: SessionManager::new(),
            users: CHashMap::new(),
            id_map: CHashMap::new()
        };
        database::auth::populate(&manager.conn).await;
        let (user_map, id_map) = crate::database::auth::get_all_users(&manager.conn).await.unwrap();
        manager.users = user_map;
        manager.id_map = id_map;
        println!("Init Manager");
        return manager;
    }

    pub async fn create_user(&self, username: String, password: String, role: String) -> Option<User> {
        let mut user = User::new(username, password, role).unwrap();

        if let Ok(id) = crate::database::auth::add_user_to_db(&self.conn, user.clone()).await {
            user.id = id;

            return Some(user)
        } else {return None}

    }

    pub async fn add_user(&self, mut user: User) -> Option<User> {
        if let Ok(id) = crate::database::auth::add_user_to_db(&self.conn, user.clone()).await {
            user.id = id;

            return Some(user)
        } else {return None}
        
    }

    pub fn modify_user(&self, user: User) {

    }

    fn set_auth_key(&self, user_id: usize) {
        let secret = rand_string(32);
        self.session.insert(user_id, secret, 60*15);
    }

    pub fn is_auth(&self, session: LoginSession) -> Option<User> {
        if let Some(secret) = self.session.get(session.id) {
            if secret == session.auth_key {
                let username: String = self.id_map.get(&session.id).unwrap().clone();
                let user = self.users.get_mut(&username).unwrap().clone();

                return Some(user)
            }
            return None;
        } else {
            None
        }
    }

    pub fn login(&self, username: String, password: String) -> Result<(), LoginError> {
        
        if let Some(mut user) = self.users.get_mut(&username) {
            match user.check_password(password) {
                Ok(true) => {
                    self.set_auth_key(user.id)
                }
                Ok(false) => {
                    println!("no user");
                    return Err(LoginError::NoUser);
                }
                Err(_) => {
                    return Err(LoginError::InternalError);
                }
            }
        } else {
            //Make a better error!
            println!("no user");
            return Err(LoginError::NoUser);
        }

        Ok(())
    }

    pub fn logout(&self, username: String) {
        if let Some(user) = self.users.get(&username) {
            self.session.remove(user.id);
        }
    }
}