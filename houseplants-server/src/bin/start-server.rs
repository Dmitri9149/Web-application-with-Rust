use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPool;
use state::AppState;

#[path = "../server_modules/mod.rs"]
mod server_modules;
use server_modules::{db_access, handlers, routes, state, helpers};
use routes::*;

// entry point to start server 

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

  // get host port from .env
  let port = helpers::get_server_port();
  println!("Listening on: {}", &port);

  // get client's db_url and db_pool
  let db_url = helpers::get_db_url();
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
      .configure(interesting_fact_routes)
  };

  // Start server 
  HttpServer::new(app).bind(port).unwrap().run().await
}


