use actix_web::{get, web, App, HttpServer, HttpResponse};
use crate::state::AppState;
use std::time::{SystemTime, UNIX_EPOCH};

//#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().json("Hello World!")
}

//#[get("/{name}")]
pub async fn hello(path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();
    let response = format!("Hello {}!", &name);
    HttpResponse::Ok().json(&response)

}

