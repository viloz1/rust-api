use actix_web::{post, web, Responder, HttpResponse};

use crate::communication::protocols::{
    From, Request, RequestResult, RequestType,
};

use crate::guards::auth::auth::Auth;
use crate::states::ProcessComm;
use crossbeam::channel::unbounded;

#[post("/restart/{id}")]
pub async fn restart<'a>(
    state: web::Data<ProcessComm>,
    path: web::Path<usize>,
    _auth: Auth
) -> HttpResponse {
    let id = path.into_inner();
    let (tx, rx) = unbounded::<RequestResult>();

    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::Restart,
        id: Some(id),
        answer_channel: Some(tx),
        ..Default::default()
    });

    match result {
        Err(_) => HttpResponse::InternalServerError().body("Could not start the process"),
        _ => HttpResponse::Ok().body(""),
    }

}
