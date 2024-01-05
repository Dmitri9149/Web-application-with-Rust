use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use routes::*;
use state::AppState;
use tera::Tera;

#[path = "../client_modules/mod.rs"]
mod client_modules;

// entry point to start server 

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  // get some parameters from .env file 
  // here the SERVER_PORT=localhost:3000
  dotenv().ok();
  let port = env::var("SERVER_PORT")
    .expect("Is SERVER_PORT set in .env file? From what folder you start server (where in .env file)?");
  println!("Listening on: {}", &host_port);

  let db_url = env::var("DATABASE_URL")
    .expect("Is DATABASE_URL set in .env file? From what folder you start server (where in .env file)?" );
  let db_pool = PgPool::connect(&db_url).await.unwrap();

    // server state as data 
    let shared_data = web::Data::new(AppState {
      server_is_running_message: "The server test page is running".to_string(),
      db: db_pool, 
    });

  // Construct App 
  let app = move || {
    // use tera templates 
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
    App::new()
      .app_data(Data::new(tera.clone()))
      .app_data(shared_data.clone())
      .configure(home_routes)
      .configure(general_routes)
      .configure(authorization_routes)
  };
 
  // Start server 
  HttpServer::new(app).bind(port).unwrap().run().await
}