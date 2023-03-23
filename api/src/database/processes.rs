use crossbeam::channel::unbounded;
use sqlx::{Row, SqlitePool, Error, sqlite::SqliteRow};
use log::*;

use crate::process_handler::process::{Process, ProcessStatus};

#[derive(Debug)]
pub struct ProcessSQLModel {
  pub name: String,
  pub path: String,
  pub start_cmd: String,
  pub stop_cmd: String,
  pub build_cmd: String,
  pub branch: String,
  pub git_url: String
}

pub async fn populate(pool: &SqlitePool) -> Result<(),Error> {
    sqlx::query(
		r#"
        CREATE TABLE IF NOT EXISTS Processes (
        Name text,
        Id integer primary key autoincrement,
        Path text,
        Start text,
        Stop text,
        Build text,
        GitURL text,
        Branch text,
        UNIQUE(Id)
        );"#,
    )
    .execute(pool)
    .await?;

    info!("Populated process database");

    return Ok(())
}

pub async fn add_process_to_db(pool: &SqlitePool, process: ProcessSQLModel) -> Result<(usize),Error>  {
  let row = sqlx::query("insert or ignore into Processes (Name, Path, Start, Stop, Build, GitURL, Branch) values ($1, $2, $3, $4, $5, $6, $7)")
		.bind(process.name)
    .bind(process.path)
    .bind(process.start_cmd)
    .bind(process.stop_cmd)
    .bind(process.build_cmd)
    .bind(process.git_url)
    .bind(process.branch)
		.execute(pool)
		.await?;
    return Ok(row.last_insert_rowid() as usize)
}

pub async fn update_process_in_db(pool: &SqlitePool, process: ProcessSQLModel, id: usize) -> Result<(),Error>  {
  sqlx::query(r#"
  update Processes set
    Name = ?1,
    Path = ?2,
    Start = ?3,
    Stop = ?4,
    Build = ?5,
    GitURL = ?6,
    Branch = ?7
  where Id = ?8"#
)
		.bind(process.name)
    .bind(process.path)
    .bind(process.start_cmd)
    .bind(process.stop_cmd)
    .bind(process.build_cmd)
    .bind(process.git_url)
    .bind(process.branch)
    .bind(id as i64)
		.execute(pool)
		.await?;

    return Ok(())
}

pub async fn get_all_proccesses(pool: &SqlitePool) -> Result<Vec<(usize, ProcessSQLModel)>,Error> {
  let rows = sqlx::query("SELECT * FROM Processes order by cast(Id as int)").fetch_all(pool).await?;
  let result = rows
		.iter()
		.map(|r| 
      (r.get::<i64, _ >("Id") as usize, 
        ProcessSQLModel {
          name: r.get::<String, _ >("Name"),
          path: r.get::<String, _ >("Path"),
          stop_cmd: r.get::<String, _ >("Stop"),
          start_cmd: r.get::<String, _ >("Start"),
          build_cmd: r.get::<String, _ >("Build"),
          git_url: r.get::<String, _ >("GitURL"),
          branch: r.get::<String, _ >("Branch")
        }
      ))
		.collect::<Vec<(usize, ProcessSQLModel)>>();
  return Ok(result)
}

pub async fn get_process_by_id(id: usize, pool: &SqlitePool) -> Result<(usize, ProcessSQLModel),Error> {
  let r = sqlx::query("SELECT * FROM Processes where Id = ?1").bind(id as i64).fetch_one(pool).await?;
  let result = (
    r.get::<i64, _ >("Id") as usize,
    ProcessSQLModel {
      name: r.get::<String, _ >("Name"),
      path: r.get::<String, _ >("Path"),
      stop_cmd: r.get::<String, _ >("Stop"),
      start_cmd: r.get::<String, _ >("Start"),
      build_cmd: r.get::<String, _ >("Build"),
      git_url: r.get::<String, _ >("GitURL"),
      branch: r.get::<String, _ >("Branch")
    }
  );
  return Ok(result)
} 