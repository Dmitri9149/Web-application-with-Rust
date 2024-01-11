use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use tera::Tera;

#[path = "../client_modules/mod.rs"]
mod client_modules;
use client_modules::{db_access, errors, handlers, model, routes, state, helpers};
use routes::*;


// entry point to start server (client side) 
#[actix_web::main]
async fn main() -> std::io::Result<()> {

// get host port from .env
  let port = helpers::get_host_port();
  println!("Listening on: {}", &port);

  // get client's db_url and db_pool
  let db_url = helpers::get_db_url_client();
  let db_pool = PgPool::connect(&db_url).await.unwrap();

  // server state as data 
  let shared_data = web::Data::new(state::AppState {
    web_client_is_running_message: "The web client test page is running".to_string(),
    db: db_pool, 
  });

  // Construct App 
  let app = move || {

    // use Tera templates  
    let tera = match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/**/*")) {
//    let tera = match Tera::new("./templates/**/*.html") {
      Ok(t) => t, 
      Err(e) => {
        println!("Parsing error(s): {}", e);
        ::std::process::exit(1);
      }
    };
    
    App::new()
      .app_data(Data::new(tera.clone()))
      .app_data(shared_data.clone())
      .configure(home_routes)
      .configure(general_routes)
      .configure(authorization_routes)
      .configure(plant_routes)
      .configure(member_routes)
  };
 
  // Start server 
  HttpServer::new(app).bind(port).unwrap().run().await
}