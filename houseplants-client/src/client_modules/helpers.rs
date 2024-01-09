use dotenv::dotenv;
use std::env;

// get server port from .env file 
pub fn get_server_port () -> String {
    // url of resource on server 
    dotenv().ok();
    let port = env::var("SERVER_PORT")
      .expect("Is SERVER_PORT set in .env file? Where in .env file?");
    port
}

// get server port from .env file 
pub fn get_host_port () -> String {
  // url of resource on server 
  dotenv().ok();
  let port = env::var("HOST_PORT")
    .expect("Is HOST_PORT set in .env file? Where in .env file?");
  port
}

// get server port from .env file 
pub fn get_db_url_client () -> String {
  // url of resource on server 
  dotenv().ok();
  let db_url_client = env::var("DATABASE_URL")
    .expect("Is DATABASE_URL set in .env file? Where in .env file?");
  db_url_client
}