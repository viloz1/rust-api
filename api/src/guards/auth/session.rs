use chashmap::CHashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct SessionManager {
    sessions: CHashMap<usize, AuthKey>
}

#[derive(Clone)]
pub struct AuthKey {
    secret: String,
    expires: usize
}

impl SessionManager {

    pub fn new() -> SessionManager {
        SessionManager {
            sessions: CHashMap::new()
        }
    }

    pub fn insert(&self, id: usize, key: String, expires: usize) {
        let auth = AuthKey {
            secret: key,
            expires
        };
        self.sessions.insert(id, auth);
    }

    pub fn remove(&self, id: usize) {
        self.sessions.remove(&id);
    }

    pub fn get(&self, id: usize) -> Option<String> {
        let key = self.sessions.get(&id)?;
        return Some(key.secret.clone());
    }

    pub fn clear_all(&self){
        self.sessions.clear();
    }

    pub fn clear_expired(&self) {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        self.sessions.retain(|_, auth_key| auth_key.expires > time.as_secs() as usize);
    }
}