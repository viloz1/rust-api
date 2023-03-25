use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2, Params
};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    username: String,
    role: String,
    password: String
}

impl User {

    fn get_argon_settings(&self) -> Argon2<'static> {
        Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, Params::new(12288 , 3, 1, Some(32)).unwrap())
    }

    pub fn new(username: String, password: String, role: String) -> Result<User, argon2::password_hash::Error> {
        let mut user = User {
            id: 0,
            username: username,
            role: role,
            password: "".to_string()
        };
        user.set_password(password)?;
        return Ok(user)
    }

    pub fn new_raw(username: String, password: String, role: String, id: usize) -> User {
        User {
            id,
            username,
            role,
            password
        }
    }

    pub fn set_password(&mut self, new_password: String) -> Result<(), argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = self.get_argon_settings();
        let pw_hash = argon2.hash_password(new_password.as_bytes(), &salt)?.to_string();
        self.password = pw_hash;
        Ok(())
    } 

    pub fn check_password(&mut self, password: String) -> Result<bool, argon2::password_hash::Error> {
        let password_hash = self.get_password_hash();
        let parsed_hash = PasswordHash::new(&password_hash)?;
        return Ok(self.get_argon_settings().verify_password(password.as_bytes(), &parsed_hash).is_ok());
           
    }

    pub fn get_role(&mut self) -> String {
        self.role.clone()
    }

    pub fn get_username(&mut self) -> String {
        self.username.clone()
    }

    pub fn get_password_hash(&mut self) -> String {
        self.password.clone()
    }
}