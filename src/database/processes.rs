use crossbeam::channel::unbounded;
use sqlx::{Row, SqlitePool, Error};

use crate::process_handler::process::{Process, ProcessStatus};

#[derive(Debug)]
pub struct ProcessSQLModel {
  pub name: String,
  pub path: String,
  pub start_path: String,
  pub stop_path: String,
  pub build_path: String,
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
        StartPath text,
        StopPath text,
        BuildPath text,
        GitURL text,
        Branch text,
        UNIQUE(Id)
        );"#,
    )
    .execute(pool)
    .await?;

    println!("Succesfully populated db");

    return Ok(())
}

pub async fn add_process_to_db(pool: &SqlitePool, process: ProcessSQLModel) -> Result<(),Error>  {
  sqlx::query("insert or ignore into Processes (Name, Path, StartPath, StopPath, BuildPath, GitURL, Branch) values ($1, $2, $3, $4, $5, $6, $7)")
		.bind(process.name)
    .bind(process.path)
    .bind(process.start_path)
    .bind(process.stop_path)
    .bind(process.build_path)
    .bind(process.git_url)
    .bind(process.branch)
		.execute(pool)
		.await?;

    return Ok(())
}

pub async fn update_process_in_db(pool: &SqlitePool, process: ProcessSQLModel, id: usize) -> Result<(),Error>  {
  sqlx::query(r#"
  update Processes set
    Name = ?1,
    Path = ?2,
    StartPath = ?3,
    StopPath = ?4,
    BuildPath = ?5,
    GitURL = ?6,
    Branch = ?7
  where Id = ?8"#
)
		.bind(process.name)
    .bind(process.path)
    .bind(process.start_path)
    .bind(process.stop_path)
    .bind(process.build_path)
    .bind(process.git_url)
    .bind(process.branch)
    .bind(id as i64)
		.execute(pool)
		.await?;

    return Ok(())
}

pub async fn get_all_proccesses(pool: &SqlitePool) -> Result<Vec<(usize, ProcessSQLModel)>,Error> {
  let rows = sqlx::query("SELECT * FROM Processes order by Id").fetch_all(pool).await?;
  let result = rows
		.iter()
		.map(|r| 
      (r.get::<i64, _ >("Id") as usize, 
        ProcessSQLModel {
          name: r.get::<String, _ >("Name"),
          path: r.get::<String, _ >("Path"),
          stop_path: r.get::<String, _ >("StopPath"),
          start_path: r.get::<String, _ >("StartPath"),
          build_path: r.get::<String, _ >("BuildPath"),
          git_url: r.get::<String, _ >("GitURL"),
          branch: r.get::<String, _ >("Branch")
        }
      ))
		.collect::<Vec<(usize, ProcessSQLModel)>>();
  return Ok(result)
}