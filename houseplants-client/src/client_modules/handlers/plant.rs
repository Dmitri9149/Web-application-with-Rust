use crate::db_access::{get_user_db, post_new_user_db};
use crate::model::{NewPlant, NewPlantResponse, 
  UpdatePlant, UpdatePlantResponse, NewPlantForm};
use crate::state::AppState;
use actix_web::{web, Error, HttpResponse, Result};
use crate::errors::CustomError;
use serde_json::json;

pub async fn show_new_plant_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
  let mut ctx = tera::Context::new();
  ctx.insert("error", "");
  ctx.insert("current_member_name", "");
  ctx.insert("current_plant_name", "");
  ctx.insert("current_plant_description", "");
  ctx.insert("current_plant_care", "");
  ctx.insert("current_plant_alternative_name", "");
  ctx.insert("current_plant_care_difficulty", "");
  ctx.insert("current_plant_price", "");
  ctx.insert("current_plant_extra_info", "");
  let s = tmpl
      .render("new_plant_form/new_plant.html", &ctx)
      .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// add new plant record , to be used from New Plant Form 
pub async fn new_plant_addition(
  tmpl: web::Data<tera::Tera>,
  app_state: web::Data<AppState>,
  params: web::Form<NewPlantForm>,
) -> Result<HttpResponse, Error> {
  let mut ctx = tera::Context::new();
  let username = params.member_name.clone();
  let user = get_user_db(&app_state.db, 
    username.to_string()).await;
  if let Ok(user) = user {
    let new_plant = json!({
      "member_id": user.member_id, 
      "plant_name": &params.plant_name,
      "plant_description": &params.plant_description,
      "plant_care": &params.plant_care,
      "plant_alternative_name": &params.plant_alternative_name,
      "plant_care_difficulty": &params.plant_care_difficulty,
      "plant_price": &params.plant_price,
      "plant_extra_info": &params.plant_extra_info,
    });
    let member_id = user.member_id.unwrap();
    let awc_client = awc::Client::default();
    let res = awc_client
                .post("http://localhost:3000/plants/")
                .send_json(&new_plant)
                .await
                .unwrap()
                .body()
                .await?;
    println!("Finished call from DB with new plant as result: {:?}", res);
    let plant_response: NewPlantResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
    Ok(HttpResponse::Ok().json(plant_response))
  } else {
    let s = tmpl
    .render("new_plant_form/new_plant.html", &ctx)
    .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
  }
}

// add new plant record if web::Path is given , 
// can be used with curl 
pub async fn handle_insert_plant(
  _tmpl: web::Data<tera::Tera>,
  _app_state: web::Data<AppState>,
  path: web::Path<i32>,
  params: web::Json<NewPlant>
) -> Result<HttpResponse, Error> {
  let member_id = path.into_inner();
  let new_plant = json!({
    "member_id": member_id,
    "plant_name": &params.plant_name,
    "plant_description": &params.plant_description,
    "plant_care": &params.plant_care,
    "plant_care_difficulty": &params.plant_care_difficulty,
    "plant_alternative_name": &params.plant_alternative_name,
    "plant_price": &params.plant_price,
    "plant_extra_info": &params.plant_extra_info
  });
  println!("New plant {:?}", new_plant);
  let awc_client = awc::Client::default();
  let res = awc_client
              .post("http://localhost:3000/plants/")
              .send_json(&new_plant)
              .await
              .unwrap()
              .body()
              .await?;

  println!("Finished call: {:?}", res);
  let plant_response: NewPlantResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
  Ok(HttpResponse::Ok().json(plant_response))
}

// delete a plant for a member_id
pub async fn handle_delete_plant(
  _tmpl: web::Data<tera::Tera>,
  _app_state: web::Data<AppState>,
  path: web::Path<(i32, i32)>
) -> Result<HttpResponse, Error> {
  let (member_id, plant_id) = path.into_inner();
  let awc_client = awc::Client::default();
  let delete_url = format!("http://localhost:3000/plants/{}/{}", member_id, plant_id);
  let _res = awc_client.delete(delete_url).send().await.unwrap();
  Ok(HttpResponse::Ok().body("Plant record deleted "))
}
