use crate::db_access::{get_user_db, post_new_user_db};
use crate::model::{NewPlant, NewPlantResponse, 
  UpdatePlant, UpdatePlantResponse, NewPlantForm};
use crate::state::AppState;
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;