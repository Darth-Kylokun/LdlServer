use actix::Addr;
use actix_web::{get, Scope, HttpResponse, Error, HttpRequest, web};
use actix_web_actors::ws;
use std::time::Instant;

use crate::{server, session};

#[get("/connect")]
async fn connect(req: HttpRequest, stream: web::Payload, srv: web::Data<Addr<server::ChatServer>>) -> Result<HttpResponse, Error> {
    log::info!("here");
    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "0".to_owned(),
            name: None,
            addr: srv.get_ref().clone()
        },
        &req,
        stream
    )
}

pub fn launch() -> Scope {
    web::scope("/ws")
        .service(connect)
}