use crate::handlers::{home::*, general::*};
use crate::handlers::authorization::*;
use crate::handlers::plant::*;
use crate::handlers::member::*;
use crate::handlers::interesting_fact::*;
use actix_files as fs;
use actix_web::web;

// home page route
pub fn home_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/home")
      .route("/", web::get().to(index))
      .route("/{name}", web::get().to(hello))
  );
}

// general route to test some functionalities 
pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/general").route("/", web::get().to(general_page_handler))
  );
}


// user authorization routes 
pub fn authorization_routes(config: &mut web::ServiceConfig) {
  config.service(
  web::scope("/auth")
    .service(fs::Files::new(
      "/templates", "./templates").show_files_listing())
    .service(web::resource("/").route(web::get().to(show_register_form)))
    .service(web::resource("/signinform").route(web::get().to(show_signin_form)))
    .service(web::resource("/signin").route(web::post().to(handle_signin)))
//    .service(web::resource("/register").route(web::get().to(handle_register_redirect)))
    .service(web::resource("/register").route(web::post().to(handle_register)))
  );
}


// create / delete / modify plants records from the client 
pub fn plant_routes(config: &mut web::ServiceConfig) {
  config.service(
  web::scope("/plants")
      .service(web::resource("show/{member_id}/{plant_id}")
        .route(web::get().to(show_plant_for_member_render_template)))
      .service(web::resource("/").route(web::get().to(show_new_plant_form)))
      .service(web::resource("add_new").route(web::post().to(new_plant_addition)))
      .service(web::resource("new/{member_id}")
        .route(web::post().to(handle_insert_plant)))
      .service(web::resource("update/{member_id}/{plant_id}")
        .route(web::put().to(handle_update_plant)))
      .service(web::resource("delete/{member_id}/{plant_id}")
        .route(web::delete().to(handle_delete_plant)))
        .service(web::resource("get_all").route(web::get().to(show_plants)))
  );
}

// manage members records  
pub fn member_routes(config: &mut web::ServiceConfig) {
  config.service(
  web::scope("/members")
      .service(web::resource("/").route(web::get().to(show_members)))
  );
}

// routes for handling members 
pub fn interesting_fact_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/facts")
      .service(web::resource("/").route(web::get().to(show_interesting_facts)))
  );
}


