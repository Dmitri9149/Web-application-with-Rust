use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Member {
  pub member_id: i32,
  pub member_name: String, 
  pub member_info: String, 
}