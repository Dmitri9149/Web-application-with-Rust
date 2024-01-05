use crate::db_access::{get_user_record_db};
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

pub async fn handle_register(
  tmpl: web::Data<tera::Tera>,
  app_state: web::Data<AppState>,
  params: web::Form<MemberRegisterForm>
) -> Result<HttpResponse, Error> {
  let mut ctx = tera::Context::new();
  let s;
  let username = params.username.clone();
  let user = get_user_record_db(&app_state.db, username.to_string()).await;
  let user_not_found: bool = user.is_err();

  // If user is not found in database, proceed to verification of passwords
  if user_not_found {
    if params.password != params.confirmation {
      ctx.insert("error", "Passwords do not match");
      ctx.insert("current_username", &params.username);
      ctx.insert("current_password", "");
      ctx.insert("current_confirmation", "");
      ctx.insert("current_name", &params.name);
      ctx.insert("current_info", &params.info);
      s = tmpl
            .render("register_form/register.html", &ctx)
            .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
    } else {
      let new_member = json!({
        "member_name": &params.name,
        "member_info": &params.info
      });
      let awc_client = awc::Client::default();
      let result = awc_client
                  .post("http://localhost:3000/members/")
                  .send_json(&new_member)
                  .await
                  .unwrap()
                  .body()
                  .await?;
      let member_response: MemberResponse = serde_json::from_str(&std::str::from_utf8(&result)?)?;
      s = format!(
        "Congratulations. You have been successfully registered as member of Houseplants and your member id is: {}. To start using HousePlant, please login with your credentials.", 
        member_response.member_id
      );

      // Hashing the password to store it in DB 
      let salt = b"somerandomsalt";
      let config = Config::default();
      let hash = argon2::hash_encoded(
        params.password.clone().as_bytes(),
        salt,
        &config).unwrap();
      let user = User {
        username,
        member_id: Some(member_response.member_id),
        user_password: hash,
      };
      let _member_created = post_new_user_db(&app_state.db, user).await?;
    }
  } else {
      ctx.insert("error", "User Id already exists");
      ctx.insert("current_username", &params.username);
      ctx.insert("current_password", "");
      ctx.insert("current_confirmation", "");
      ctx.insert("current_name", &params.name);
      ctx.insert("current_info", &params.info);
      s = tmpl
          .render("register_form/register.html", &ctx)
          .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
 }


