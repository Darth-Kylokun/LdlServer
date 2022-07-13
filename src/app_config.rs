use actix_web::web;
use crate::handlers::{index};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        index::hello
    );
}