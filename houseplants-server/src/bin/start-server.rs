use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use routes::*;
use state::AppState;


#[path = "../server_modules/db_access/mod.rs"]
mod db_access;
#[path = "../server_modules/handlers/mod.rs"]
mod handlers;
#[path = "../server_modules/models/mod.rs"]
mod models;
#[path = "../server_modules/routes.rs"]
mod routes;
#[path = "../server_modules/state.rs"]
mod state;
#[path = "../server_modules/errors.rs"]
mod errors;

// entry point to start server 

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  // get some parameters from .env file 
  // here the SERVER_PORT=localhost:3000
  dotenv().ok();
  let db_url = env::var("DATABASE_URL")
    .expect("Is DATABASE_URL set in .env file? From what folder you start server (where in .env file)?" );
  let db_pool = PgPool::connect(&db_url).await.unwrap();

  let shared_data = web::Data::new(AppState {
    server_is_running_message: "The server test page is running".to_string(),
    db: db_pool, 
  });

  // Construct App 
  let app = move || {
    App::new()
      .app_data(shared_data.clone())
      .configure(home_routes)
      .configure(general_routes)
      .configure(member_routes)
      .configure(plant_routes)
  };
  let port = env::var("SERVER_PORT")
    .expect("Is SERVER_PORT set in .env file? Where is the .env file located?");
  println!("Listening on: {}", &port);

  // Start server 
  HttpServer::new(app).bind(port).unwrap().run().await
}


