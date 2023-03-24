use sqlx::{Pool, Sqlite};
use crate::guards::auth::session::SessionManager;
use crate::guards::auth::user::User;

struct UsersManager {
    conn: Pool<Sqlite>,
    session: SessionManager
}

impl UsersManager {
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
}