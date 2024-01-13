use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Fact {
  pub member_id: i32,
  pub fact_id: i32,
  pub content: String,
  pub ref_to_origin: Option<String>,
  pub posted_time: Option<NaiveDateTime>
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewFact {
  pub member_id: i32,
  pub content: String,
  pub ref_to_origin: Option<String>,
}

impl From<web::Json<NewFact>> for NewFact {
  fn from(new_fact: web::Json<NewFact>) -> Self {
    NewFact {
      member_id: new_fact.member_id,
      content: new_fact.content.clone(),
      ref_to_origin: new_fact.ref_to_origin.clone(),
    }
  }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateFact {
  pub content: Option<String>,
  pub ref_to_origin: Option<String>,
}

impl From<web::Json<UpdateFact>> for UpdateFact {
  fn from(update_fact: web::Json<UpdateFact>) -> Self {
    UpdateFact {
      content: update_fact.content.clone(),
      ref_to_origin: update_fact.ref_to_origin.clone(),
    }
  }
}