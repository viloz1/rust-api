use actix_web::{post, web, HttpResponse};

use crate::communication::protocols::{
    From, Request, RequestResult, RequestType,
};

use crate::guards::auth::auth::Auth;
use crate::states::ProcessComm;
use crossbeam::channel::unbounded;

#[post("/stop/{id}")]
pub async fn stop<'a>(
    state: web::Data<ProcessComm>,
    path: web::Path<usize>,
    _auth: Auth
) -> HttpResponse {
    let id = path.into_inner();
    let (tx, rx) = unbounded::<RequestResult>();

    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::Stop,
        id: Some(id),
        answer_channel: Some(tx),
        ..Default::default()
    });

    match result {
        Err(_) => HttpResponse::InternalServerError().body("Could not start the process"),
        _ => HttpResponse::Ok().body(""),
    }

}
