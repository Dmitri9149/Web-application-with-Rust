use crate::handlers::{home::*};
use actix_web::web;

pub fn home(cfg: &mut web::ServiceConfig) {
  cfg.service(index).service(hello);
}


