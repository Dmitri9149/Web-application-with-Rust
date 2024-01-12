use crate::db_access::{get_user_db, post_new_user_db};
use crate::model::{MemberRegisterForm, MemberResponse, User, MemberSigninForm};
use crate::state::AppState;
use actix_web::{web, Error, HttpResponse, Result};
use crate::errors::CustomError;
use serde_json::json;
use crate::helpers::{get_server_port};

// view list of all members of House Plants 
// Tera template is used for handling the view 
pub async fn show_members(
  tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
  let awc_client = awc::Client::default();
  let members_url = format!("http://{}/members/", get_server_port());
  let res = awc_client
              .get(members_url)
              .send()
              .await
              .unwrap()
              .body()
              .await?;
  let members_response: Vec<MemberResponse> = serde_json::from_str(&std::str::from_utf8(&res)?)?;
  println!("Members {:?}", &members_response);
  let mut ctx = tera::Context::new();
    ctx.insert("members", &members_response);
/*    ctx.insert("current_alternative_name", &plant_response.plant_alternative_name);
    ctx.insert("current_description", &plant_response.plant_description);
    ctx.insert("current_care", &plant_response.plant_care);
    ctx.insert("current_care_difficulty", &plant_response.plant_care_difficulty);
    ctx.insert("current_price", &plant_response.plant_price);
    ctx.insert("current_extra_info", &plant_response.plant_extra_info); */ 
  let s = tmpl
    .render("members/members.html", &ctx)
    .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}