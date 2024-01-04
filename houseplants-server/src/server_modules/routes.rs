use crate::handlers::{home::*, general::*, member::*, plant::*};
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
    .route("/", web::post().to(post_new_member))
    .route("/", web::get().to(get_members))
  );
}

pub fn plant_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
  web::scope("/plants")
  .route("/", web::post().to(post_new_plant))
  .route("/{member_id}", web::get().to(get_plants_for_member))
//  .route("/{member_id}/{plant_id}", web::get().to(get_plant_details))
//  .route("/{member_id}/{plant_id}", web::put().to(update_plant_details))
//  .route("/{member_id}/{plant_id}", web::delete().to(delete_plant))
  );
}



