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

#[derive(Deserialize, Debug, Clone)]
pub struct NewPlant {
  pub member_id: i32,
  pub plant_name: String,
  pub plant_description: Option<String>,
  pub plant_care: Option<String>,
  pub plant_care_difficulty: Option<String>,
  pub plant_alternative_name: Option<String>,
  pub plant_price: Option<i32>,
  pub plant_extra_info: Option<String>,
}

impl From<web::Json<NewPlant>> for NewPlant {
  fn from(new_plant: web::Json<NewPlant>) -> Self {
    NewPlant {
      member_id: new_plant.member_id,
      plant_name: new_plant.plant_name.clone(),
      plant_description: new_plant.plant_description.clone(),
      plant_care: new_plant.plant_care.clone(),
      plant_care_difficulty: new_plant.plant_care_difficulty.clone(),
      plant_extra_info: new_plant.plant_extra_info.clone(),
      plant_alternative_name: new_plant.plant_alternative_name.clone(),
      plant_price: new_plant.plant_price,
    }
  }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdatePlant {
  pub plant_name: Option<String>,
  pub plant_description: Option<String>,
  pub plant_care: Option<String>,
  pub plant_care_difficulty: Option<String>,
  pub plant_alternative_name: Option<String>,
  pub plant_price: Option<i32>,
  pub plant_extra_info: Option<String>,
}

impl From<web::Json<UpdatePlant>> for UpdatePlant {
  fn from(update_plant: web::Json<UpdatePlant>) -> Self {
    UpdatePlant {
      plant_name: update_plant.plant_name.clone(),
      plant_description: update_plant.plant_description.clone(),
      plant_care: update_plant.plant_care.clone(),
      plant_care_difficulty: update_plant.plant_care_difficulty.clone(),
      plant_extra_info: update_plant.plant_extra_info.clone(),
      plant_alternative_name: update_plant.plant_alternative_name.clone(),
      plant_price: update_plant.plant_price,
    }
  }
}