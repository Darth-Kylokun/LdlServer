use actix_web::web;
use crate::handlers::{user, ws};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(user::launch())
        .service(ws::launch());
}