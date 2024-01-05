use crate::handlers::{home::*, general::*};
use crate::handlers::authorization::{show_register_form, 
  show_signin_form, handle_register};
use actix_web::web;

// home page route
pub fn home_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/home")
    .service(fs::Files::new(
      "/static", "./static").show_files_listing())
    .route("", web::get().to(index))
    .route("/{name}", web::get().to(hello))
  );
}

// general route to test some functionalities 
pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("general", web::get().to(general_page_handler));
}

// user authorization routes 
pub fn authorization(config: &mut web::ServiceConfig) {
  config.service(
  web::scope("")
  .service(fs::Files::new(
    "/static", "./static").show_files_listing())
  .service(web::resource("/").route(web::get().to(show_register_form)))
  .service(web::resource("/signinform").route(web::get().to(show_signin_form)))
//  .service(web::resource("/signin").route(web::post().to(handle_signin)))
  .service(web::resource("/register").route(web::post().to(handle_register)))
  );
}

