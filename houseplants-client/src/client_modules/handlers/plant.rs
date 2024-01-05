use crate::db_access::{get_user_db, post_new_user_db};
use crate::model::{NewPlant, NewPlantResponse, 
  UpdatePlant, UpdatePlantResponse, NewPlantForm};
use crate::state::AppState;
use actix_web::{web, Error, HttpResponse, Result};
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
      .render("new_plant_form/new_plant_form.html", &ctx)
      .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

