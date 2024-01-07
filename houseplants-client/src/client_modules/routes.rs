use crate::handlers::{home::*, general::*};
use crate::handlers::authorization::{
  show_register_form,show_signin_form, handle_register, handle_signin};
use crate::handlers::plant::{show_new_plant_form, new_plant_addition};
use actix_files as fs;
use actix_web::web;

// home page route
pub fn home_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("home")
      .route("", web::get().to(index))
      .route("/{name}", web::get().to(hello))
  );
}

// general route to test some functionalities 
pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("general", web::get().to(general_page_handler));
}

// user authorization routes 
pub fn templates_routes(config: &mut web::ServiceConfig) {
  config.service(
  web::scope("")
    .service(fs::Files::new(
      "/templates", "./templates").show_files_listing()));
}

// user authorization routes 
pub fn authorization_routes(config: &mut web::ServiceConfig) {
  config.service(
  web::scope("auth")
/*    .service(fs::Files::new(
      "/templates", "./templates").show_files_listing()) */
    .service(web::resource("/").route(web::get().to(show_register_form)))
    .service(web::resource("/signinform").route(web::get().to(show_signin_form)))
    .service(web::resource("/signin").route(web::post().to(handle_signin)))
    .service(web::resource("/register").route(web::post().to(handle_register)))
  );
}


// create / delete / modify plants records from the client 
pub fn plant_routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("plants")
//          .service(
//              web::resource("show/{member_id}/{plant_id}").route(web::get().to(handle_show_member_plant)))
      .service(web::resource("/").route(web::get().to(show_new_plant_form)))
      .service(web::resource("add_new").route(web::post().to(new_plant_addition)))
//          .service(web::resource("new/{member_id}").route(web::post().to(handle_insert_plant)))
//          .service(
//              web::resource("{member_id}/{plant_id}").route(web::put().to(handle_update_plant)),
//          )
//          .service(
//              web::resource("delete/{member_id}/{plant_id}")
//                  .route(web::delete().to(handle_delete_plant)),
//          ),
  );
}


