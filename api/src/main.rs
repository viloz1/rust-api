use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::thread;
use std::*;

//use database::processes::ProcessSQLModel;
//use process_handler::process::{Process, ProcessStatus};
use log::info;

use futures::TryStreamExt;

use database::processes::ProcessSQLModel;
use process_handler::process::{Process, ProcessStatus};

use crossbeam::channel::unbounded;
use ctrlc;
use sqlx::SqlitePool;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod communication;
mod process_handler;
mod database;
use process_handler::ProcessHandler;

//use crate::guards::cors::CORS;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[allow(unused_must_use)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Starting API...");
    if !std::path::Path::new("databases").exists() {
        std::fs::create_dir("databases");
    }

    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();


    if !Path::new("databases/auth.db").exists() {
        File::create("databases/auth.db");
    }

    if !Path::new("databases/processes.db").exists() {
        File::create("databases/processes.db");
    }
    
    let conn = SqlitePool::connect("databases/auth.db").await?;

    let process_db_pool = SqlitePool::connect("databases/processes.db").await?;
    database::processes::populate(&process_db_pool).await?;

    let result = database::processes::get_all_proccesses(&process_db_pool).await?;

    //The channel that Rocket will listen to
    ctrlc::set_handler(move || {
        info!("Recieved CTRLC, shutting down");
    })
    .expect("Error setting CTRLC");
    //The channel that process_handler will listen too
    let (tx, rx) = unbounded();
    let pool = process_db_pool.clone();
    thread::spawn(move || {
        let mut proc_handler: ProcessHandler = ProcessHandler::new(rx, &process_db_pool);
        proc_handler.start(&process_db_pool);
    });
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
