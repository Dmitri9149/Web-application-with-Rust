use actix_web::{get, web, Error, App, HttpServer, HttpResponse};
use crate::state::AppState;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::errors::CustomError;

// handlers to 'home' scope 

pub async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
//    HttpResponse::Ok().json("Hello World!")
    let ctx = tera::Context::new();
    let s = tmpl
        .render("home/home.html", &ctx)
        .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn hello(path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();
    let response = format!("Hello {}!", &name);
    HttpResponse::Ok().json(&response)

}
