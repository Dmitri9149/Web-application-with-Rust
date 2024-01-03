use crate::handlers::{home::*, general::*};
use actix_web::web;

pub fn home_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/home")
    .route("", web::get().to(index))
    .route("/{name}", web::get().to(hello))
  );
}

pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("general", web::get().to(general_page_handler));
}



