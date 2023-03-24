use std::time::{SystemTime, UNIX_EPOCH};

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