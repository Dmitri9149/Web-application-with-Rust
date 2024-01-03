use crate::db_access::plant::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
// use core::error::Error;
use std::error::Error;


pub async fn get_plants_for_member(
  app_state: web::Data<AppState>,
  path: web::Path<(i32,)>) -> HttpResponse {
    let (member_id, )  = path.into_inner();
    let plants = get_plants_for_member_db(&app_state.db, member_id)
      .await;
    HttpResponse::Ok().json(plants)
}