use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::thread;
use std::*;

//use database::processes::ProcessSQLModel;
//use process_handler::process::{Process, ProcessStatus};
use log::info;

use futures::{TryStreamExt, executor};

use database::processes::ProcessSQLModel;
use process_handler::process::{Process, ProcessStatus};

use crossbeam::channel::unbounded;
use ctrlc;
use sqlx::sqlite::SqlitePool;

use actix_web::{get, post, http, web, Responder, App, HttpRequest, HttpResponse, HttpServer};
use actix_cors::Cors;

use states::{ProcessComm, DBConnections};

mod communication;
mod endpoints;
mod states;
mod process_handler;
mod database;
mod guards;
use process_handler::ProcessHandler;

use crate::guards::auth::users::UserManager;

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
async fn main() -> Result<(), io::Error>{
    info!("Starting API...");
    if !std::path::Path::new("databases").exists() {
        std::fs::create_dir("databases");
    }

    //log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();


    if !Path::new("databases/auth.db").exists() {
        File::create("databases/auth.db");
    }

    if !Path::new("databases/processes.db").exists() {
        File::create("databases/processes.db");
    }
    let process_db_pool = SqlitePool::connect("databases/processes.db").await.unwrap();
    database::processes::populate(&process_db_pool).await;

    //The channel that Rocket will listen to
    ctrlc::set_handler(move || {
        info!("Recieved CTRLC, shutting down");
    })
    .expect("Error setting CTRLC");
    let pool = process_db_pool.clone();

    //The channel that process_handler will listen too
    let (tx, rx) = unbounded();
    thread::spawn(move || {
        let mut proc_handler: ProcessHandler = ProcessHandler::new(rx, &process_db_pool);
        proc_handler.start(&process_db_pool);
    });

    let conn = executor::block_on(SqlitePool::connect("databases/auth.db"));
    let usermanager = executor::block_on(UserManager::connect_db(conn.unwrap()));

    
    HttpServer::new(move || {


        let cors = Cors::default()
              .allowed_origin("http://localhost:4200")
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .supports_credentials()
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(ProcessComm{sender: tx.clone()}))
            .app_data(web::Data::new(usermanager.clone()))
            .app_data(web::Data::new(DBConnections{process: pool.to_owned()}))
            .app_data(guards::auth::auth::Auth{user: None})
            .service(hello)
            .service(echo)
            .service(endpoints::add_services())
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 1337))?
    .run()
    .await
}
