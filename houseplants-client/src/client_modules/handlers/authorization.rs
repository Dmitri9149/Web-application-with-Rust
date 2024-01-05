//use crate::db_access::{};
use crate::errors::CustomError;
use crate::client_modules::state::AppState;
use crate::model::{MemberRegisterForm, MemberResponse, User, MemberSigninForm};
use actix_web::{web, Error, HttpResponse, Result};
use argon2::{self, Config};
use serde_json::json;

// show form for member registration 
pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
  let mut ctx = tera::Context::new();
  ctx.insert("error", "");
  ctx.insert("current_username", "");
  ctx.insert("current_password", "");
  ctx.insert("current_confirmation", "");
  ctx.insert("current_name", "");
  ctx.insert("current_info", "");
  let s = tmpl
      .render("register_form/register.html", &ctx)
      .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// show form for signin of already registered member
pub async fn show_signin_form(tmpl: web::Data<tera::Tera>) -> 
Result<HttpResponse, Error> {
  let mut ctx = tera::Context::new();
  ctx.insert("error", "");
  ctx.insert("current_name", "");
  ctx.insert("current_password", "");
  let s = tmpl
            .render("signin_form/signin.html", &ctx)
            .map_err(|_| CustomError::TeraError(
              "Template error".to_string()))?;

  Ok(HttpResponse::Ok().content_type("text.html").body(s))
}
