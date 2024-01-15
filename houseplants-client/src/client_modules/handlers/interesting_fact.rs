use crate::model::{NewFactResponse};
use actix_web::{web, Error, HttpResponse, Result};
use crate::errors::CustomError;
use crate::helpers::{get_server_port};

// view list of all plant records in House Plants 
// Tera template is used for handling the view 
pub async fn show_interesting_facts(
  tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
  let awc_client = awc::Client::default();
  let facts_url = format!("http://{}/facts/", get_server_port());
  let res = awc_client.get(facts_url).send()
              .await
              .unwrap()
              .body()
              .await?;
  let facts_response: Vec<NewFactResponse> = serde_json::from_str(&std::str::from_utf8(&res)?)?;
  let mut ctx = tera::Context::new();
    ctx.insert("facts", &facts_response); 
  let s = tmpl
    .render("facts/facts.html", &ctx)
    .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}