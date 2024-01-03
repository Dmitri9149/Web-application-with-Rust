use crate::handlers::{home::*, general::*, member::*};
use actix_web::web;

// home page route
pub fn home_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/home")
    .route("", web::get().to(index))
    .route("/{name}", web::get().to(hello))
  );
}

// general route to test some functionalities 
pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("general", web::get().to(general_page_handler));
}

pub fn member_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/members")
      .route("/", web::get().to(get_members))
  );
}



