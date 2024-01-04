use crate::db_access::plant::*;
use crate::models::plant::{NewPlant};
use crate::state::AppState;
use actix_web::{web, HttpResponse};



pub async fn get_plants_for_member(
  app_state: web::Data<AppState>,
  path: web::Path<(i32,)>) -> HttpResponse {
    let (member_id, )  = path.into_inner();
    let plants = get_plants_for_member_db(&app_state.db, member_id)
      .await;
    HttpResponse::Ok().json(plants)
}

pub async fn post_new_plant(
  new_plant: web::Json<NewPlant>,
  app_state: web::Data<AppState>,
) -> HttpResponse {
  let plant = post_new_plant_db(&app_state.db, NewPlant::from(new_plant))
      .await;
  HttpResponse::Ok().json(plant)
}