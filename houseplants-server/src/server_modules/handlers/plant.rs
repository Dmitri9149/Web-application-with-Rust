use crate::db_access::plant::*;
use crate::server_modules::model::plant::{NewPlant, UpdatePlant};
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

pub async fn get_plant_details(
  app_state: web::Data<AppState>,
  path: web::Path<(i32, i32)>,
  ) -> HttpResponse {
    let (member_id, plant_id)  = path.into_inner();
    let plant = get_plant_details_db(
      &app_state.db, member_id, plant_id
    ).await;
    HttpResponse::Ok().json(plant)
}

pub async fn post_new_plant(
  new_plant: web::Json<NewPlant>,
  app_state: web::Data<AppState>,
) -> HttpResponse {
  let plant = post_new_plant_db(&app_state.db, NewPlant::from(new_plant))
      .await;
  HttpResponse::Ok().json(plant)
}

pub async fn delete_plant(
  app_state: web::Data<AppState>,
  path: web::Path<(i32,i32)>,
) -> HttpResponse {
  let (member_id, plant_id) = path.into_inner();
  let plant = delete_plant_db(&app_state.db, member_id, plant_id)
    .await;
  HttpResponse::Ok().json(plant)
}

pub async fn update_plant (
  app_state: web::Data<AppState>, 
  update_plant: web::Json<UpdatePlant>,
  path: web::Path<(i32, i32)>
) -> HttpResponse {
  let (member_id, plant_id) = path.into_inner();
  let plant = update_plant_details_db(&app_state.db, member_id, 
    plant_id, update_plant.into())
    .await; 
  HttpResponse::Ok().json(plant)
}