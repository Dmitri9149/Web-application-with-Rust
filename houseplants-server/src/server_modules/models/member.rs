use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Member {
  pub member_id: i32,
  pub member_name: String, 
  pub member_info: String, 
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewMember {
  pub member_name: String, 
  pub member_info: String,  
}

impl From<web::Json<NewMember>> for NewMember {
  fn from(new_member: web::Json<NewMember>) -> Self {
    NewMember {
      member_name: new_member.member_name.clone(),
      member_info: new_member.member_info.clone(),
    }
  }
}