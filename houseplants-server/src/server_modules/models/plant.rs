use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Plant {
  pub member_id: i32,
  pub plant_id: i32,
  pub plant_name: String,
  pub plant_description: Option<String>,
  pub plant_care: Option<String>,
  pub plant_care_difficulty: Option<String>,
  pub plant_alternative_name: Option<String>, 
  pub plant_price: Option<i32>, 
  pub plant_extra_info: Option<String>,
  pub posted_time: Option<NaiveDateTime>
}