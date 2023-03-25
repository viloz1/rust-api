use std::time::{SystemTime, UNIX_EPOCH};

use chashmap::CHashMap;
use dashmap::DashMap;
use sqlx::{Row, SqlitePool, Error, sqlite::SqliteRow};
use crate::guards::auth::{user::User, auth::Auth};
use log::*;

#[derive(Clone)]
pub struct AuthConnection {
    pool: SqlitePool
}

impl AuthConnection {

    pub async fn new(pool: SqlitePool) -> AuthConnection {
        let conn = AuthConnection {
            pool
        };
        conn.populate().await;
        conn
    }

    async fn populate(&self) -> Result<(),Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS Users (
            Username text,
            Id integer primary key autoincrement,
            Role text,
            Password text,
            LastLogin integer
            );"#,
        )
        .execute(&self.pool)
        .await?;
    
        info!("Populated auth database");
    
        return Ok(())
    }
    
    pub async fn add_user_to_db(&self, mut user: User) -> Result<usize,Error>  {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!("adding user");
        let row = sqlx::query("insert or ignore into Users (Username, Password, Role, LastLogin) values ($1, $2, $3, $4)")
            .bind(user.get_username())
            .bind(user.get_password_hash())
            .bind(user.get_role())
            .bind(0 as u32)
              .execute(&self.pool)
              .await?;
          return Ok(row.last_insert_rowid() as usize)
      }
    
    pub async fn get_all_users(&self) -> Result<(DashMap<String, User>, DashMap<usize, String>),Error> {
        let rows = sqlx::query("SELECT * FROM Users order by cast(Id as int)").fetch_all(&self.pool).await?;
        let user_map = DashMap::new();
        let id_map = DashMap::new();
    
        for r in rows {
            user_map.insert(r.get::<String, _ >("Username"), 
            User::new_raw(
                r.get::<String, _ >("Username"),
                r.get::<String, _ >("Password"),
                r.get::<String, _ >("Role"),
                r.get::<i64, _ >("Id") as usize
            ));
    
            id_map.insert(r.get::<i64, _ >("Id") as usize, r.get::<String, _ >("Username"));
        }
        return Ok((user_map, id_map))
    }
    
    pub async fn get_user_by_name(&self, username: String) -> Result<User, Error> {
        let r = sqlx::query("SELECT * FROM Users where Username = ?1").bind(username).fetch_one(&self.pool).await?;
        Ok(
            User::new_raw(
                r.get::<String, _ >("Username"), 
                r.get::<String, _ >("Password"), 
                r.get::<String, _ >("Role"), 
                r.get::<i64, _ >("Id") as usize
            )
        )
    }
    
    pub async fn get_user_by_id(&self, id: usize) -> Result<User, Error> {
        let r = sqlx::query("SELECT * FROM Users where Id = ?1").bind(id as i64).fetch_one(&self.pool).await?;
        Ok(
            User::new_raw(
                r.get::<String, _ >("Username"), 
                r.get::<String, _ >("Password"), 
                r.get::<String, _ >("Role"), 
                r.get::<i64, _ >("Id") as usize
            )
        )
    }
}