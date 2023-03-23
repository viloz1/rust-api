use actix_web::{post, web, HttpResponse};

use crate::communication::protocols::{
    From, Request, RequestResult, RequestType,
};

use crate::states::ProcessComm;
use crossbeam::channel::unbounded;

#[post("/restartpull/{id}")]
pub async fn restartpull<'a>(
    state: web::Data<ProcessComm>,
    path: web::Path<usize>
) -> HttpResponse {
    let id = path.into_inner();
    let (tx, rx) = unbounded::<RequestResult>();

    let result = state.sender.send(Request {
        from: From::Rocket,
        rtype: RequestType::RestartPull,
        id: Some(id),
        answer_channel: Some(tx),
        ..Default::default()
    });

    match result {
        Err(_) => HttpResponse::InternalServerError().body("Could not start the process"),
        _ => HttpResponse::Ok().body(""),
    }

}
