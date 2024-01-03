use crate::db_access::member::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use std::error::Error;

pub async fn get_members(app_state: web::Data<AppState>) -> 
  HttpResponse {
    let members = get_members_db(&app_state.db) 
      .await;
    HttpResponse::Ok().json(members)
}