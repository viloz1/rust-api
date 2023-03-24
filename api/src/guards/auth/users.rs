use argon2::Error;
use chashmap::CHashMap;
use sqlx::{Pool, Sqlite};
use crate::guards::auth::session::SessionManager;
use crate::guards::auth::user::User;

use super::session::AuthKey;
use rand::random;

struct UserManager {
    conn: Pool<Sqlite>,
    session: SessionManager,
    users: CHashMap<String, User>,
    id_map: CHashMap<usize, String>
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
        let (user_map, id_map) = crate::database::auth::get_all_users(&manager.conn).await.unwrap();
        manager.users = user_map;
        manager.id_map = id_map;

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

    pub fn login(&self, username: String, password: String) -> Result<(), argon2::password_hash::Error> {
        if let Some(mut user) = self.users.get_mut(&username) {
            if user.check_password(password)? {
                self.set_auth_key(user.id)
            } else {
                //Make a better error!
                return Err(argon2::password_hash::Error::Password);
            }
        } else {
            //Make a better error!
            return Err(argon2::password_hash::Error::Password);
        }

        Ok(())
    }

    pub fn logout(&self, username: String) {
        if let Some(user) = self.users.get(&username) {
            self.session.remove(user.id);
        }
    }
}