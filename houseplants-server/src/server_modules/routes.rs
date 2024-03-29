use crate::handlers::{general::*, home::*, interesting_fact::*, member::*, plant::*};
use actix_web::web;

// home page route
pub fn home_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/home")
            .route("", web::get().to(index))
            .route("/{name}", web::get().to(hello)),
    );
}

// general route to test some functionalities
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("general", web::get().to(general_page_handler));
}

// routes for handling members
pub fn member_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/members")
            .route("/", web::post().to(post_new_member))
            .route("/", web::get().to(get_members))
            .route("/{member_id}", web::get().to(get_member_details))
            .route("/{member_id}", web::delete().to(delete_member))
            .route("/{member_id}", web::put().to(update_member_details)),
    );
}

// routes for handling plants records
pub fn plant_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/plants")
            .route("/", web::post().to(post_new_plant))
            .route("/", web::get().to(get_plants))
            .route("/{member_id}", web::get().to(get_plants_for_member))
            .route("/{member_id}/{plant_id}", web::get().to(get_plant_details))
            .route("/{member_id}/{plant_id}", web::put().to(update_plant))
            .route("/{member_id}/{plant_id}", web::delete().to(delete_plant)),
    );
}

// routes for handling members
pub fn interesting_fact_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/facts").route("/", web::get().to(get_interesting_facts)));
}
