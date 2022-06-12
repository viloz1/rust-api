#[macro_use] extern crate rocket;

use std::result::Result;
use std::*;
use std::thread;
use std::path::Path;

use rocket::{get, routes};
use rocket::fs::{FileServer, NamedFile};

use rocket_dyn_templates::Template;
use rocket_auth::{prelude::Error, *};

use crossbeam::channel::{unbounded};
use sqlx::*;
use ctrlc;

mod process_handler;
mod communication;
mod website;

use website::states;
use website::pages::home;
use website::github;
use website::auth;
use process_handler::ProcessHandler;



#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/favicon.ico")).await.ok()
}

#[allow(unused_must_use)]
#[tokio::main]
async fn main() -> Result<(), Error> {

    let conn = SqlitePool::connect("database.db").await?;
    let users: Users = conn.clone().into();
    users.create_table().await?;

    //The channel that Rocket will listen to
    let (tx1,rx1) = unbounded();
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    }).expect("Error setting CTRLC");
    //The channel that process_handler will listen too
    let (tx2,rx2) = unbounded();
    let mut proc_handler = ProcessHandler::new(rx2, tx1, tx2.clone());
    thread::spawn(move || proc_handler.start());

    rocket::build()
    .attach(states::stage(tx2, rx1))
    .attach(home::stage())
    .attach(github::stage())
    .attach(auth::stage())
    .mount("/", routes![favicon])
    .mount("/public", FileServer::from("public/"))
    .attach(Template::fairing())
    .manage(conn)
    .manage(users)
    .launch()
    .await
    .unwrap();
    Ok(())
}
