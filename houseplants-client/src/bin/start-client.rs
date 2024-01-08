#[path = "../client_modules/mod.rs"]
mod client_modules;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use dotenv::dotenv;
use client_modules::{db_access, errors, handlers, model, routes, state};
use routes::{authorization_routes, plant_routes, 
  home_routes, general_routes};
use sqlx::postgres::PgPool;
use std::env;

use tera::Tera;

// entry point to start server 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // get some parameters from .env file 
  // here the SERVER_PORT=localhost:3000
  dotenv().ok();
  let port = env::var("SERVER_PORT")
    .expect("Is SERVER_PORT set in .env file? From what folder you start server (where in .env file)?");
  println!("Listening on: {}", &port);

  let db_url = env::var("DATABASE_URL")
    .expect("Is DATABASE_URL set in .env file? Where is the .env file located)?" );
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
  };
 
  // Start server 
  HttpServer::new(app).bind(port).unwrap().run().await
}