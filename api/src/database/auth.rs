use std::time::{SystemTime, UNIX_EPOCH};

use chashmap::CHashMap;
use sqlx::{Row, SqlitePool, Error, sqlite::SqliteRow};
use crate::guards::auth::user::User;
use log::*;

pub async fn populate(pool: &SqlitePool) -> Result<(),Error> {
    sqlx::query(
		r#"
        CREATE TABLE IF NOT EXISTS Users (
        Name text,
        Id integer primary key autoincrement,
        Role text,
        Password text,
        LastLogin integer
        );"#,
    )
    .execute(pool)
    .await?;

    info!("Populated auth database");

    return Ok(())
}

pub async fn add_user_to_db(pool: &SqlitePool, mut user: User) -> Result<usize,Error>  {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let row = sqlx::query("insert or ignore into Users (Name, Password, Role, LastLogin) values ($1, $2, $3, $4)")
        .bind(user.get_username())
        .bind(user.get_password_hash())
        .bind(user.get_role())
        .bind(0 as u32)
          .execute(pool)
          .await?;
      return Ok(row.last_insert_rowid() as usize)
  }

pub async fn get_all_users(pool: &SqlitePool) -> Result<(CHashMap<String, User>, CHashMap<usize, String>),Error> {
    let rows = sqlx::query("SELECT * FROM Users order by cast(Id as int)").fetch_all(pool).await?;
    let user_map = CHashMap::new();
    let id_map = CHashMap::new();

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