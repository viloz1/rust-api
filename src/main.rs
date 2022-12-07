#[macro_use]
extern crate rocket;

use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::thread;
use std::*;

use rocket::fs::{FileServer, NamedFile};
use rocket::{get, routes};

use rocket_auth::{prelude::Error, *};
use rocket_dyn_templates::Template;
use futures::TryStreamExt;

use crossbeam::channel::unbounded;
use ctrlc;
use sqlx::*;

mod communication;
mod endpoints;
mod guards;
mod process_handler;
mod states;
mod database;

use process_handler::ProcessHandler;

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/favicon.ico")).await.ok()
}

#[allow(unused_must_use)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    if !std::path::Path::new("databases").exists() {
        std::fs::create_dir("databases");
    }

    if !Path::new("databases/auth.db").exists() {
        File::create("databases/auth.db");
    }

    if !Path::new("databases/processes.db").exists() {
        File::create("databases/processes.db");
    }
    let conn = SqlitePool::connect("databases/auth.db").await?;
    let users: Users = conn.clone().into();
    users.create_table().await?;

    let process_db_pool = SqlitePool::connect("databases/processes.db").await?;
    database::processes::populate(&process_db_pool).await?;
    let mut rows = sqlx::query("SELECT * from Processes")
    .fetch(&process_db_pool);
    while let Some(row) = rows.try_next().await? {
        // map the row into a user-defined domain type
        let email: u32 = row.try_get("Id")?;
        println!("{}",email);
    }


    //The channel that Rocket will listen to
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Error setting CTRLC");
    //The channel that process_handler will listen too
    let (tx, rx) = unbounded();
    thread::spawn(move || {
        let mut proc_handler: ProcessHandler = ProcessHandler::new(rx);
        proc_handler.start();
    });
    /*
    rocket::build()
        .attach(states::stage(tx, 5))
        .attach(endpoints::stage())
        .manage(conn)
        .manage(users)
        .launch()
        .await
        .unwrap();
    */
    Ok(())
}
