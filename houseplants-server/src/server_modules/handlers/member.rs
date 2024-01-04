use crate::db_access::member::*;
use crate::models::member::{NewMember};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_members(app_state: web::Data<AppState>) -> 
  HttpResponse {
    let members = get_members_db(&app_state.db) 
      .await;
    HttpResponse::Ok().json(members)
}

pub async fn post_new_member(
  new_member: web::Json<NewMember>,
  app_state: web::Data<AppState>
) -> HttpResponse {
  let member = post_new_member_db(&app_state.db, NewMember::from(new_member))
    .await;
  HttpResponse::Ok().json(member)
}

pub async fn delete_member(
  app_state: web::Data<AppState>,
  path: web::Path<(i32,)>) -> HttpResponse {
  let (member_id,) = path.into_inner();
  let member = delete_member_db(&app_state.db, member_id) 
    .await;
  HttpResponse::Ok().json(member)
}