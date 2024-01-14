use dotenv::dotenv;
use std::env;

// get server port from .env file 
pub fn get_server_port () -> String {
    // url of resource on server 
    dotenv().ok();
    let port = env::var("SERVER_PORT")
      .expect("Is SERVER_PORT set in .env file? Check where is .env file?");
    port
}

// get server port from .env file 
pub fn get_db_url_server () -> String {
  // url of resource on server 
  dotenv().ok();
  let db_url_client = env::var("DATABASE_URL")
    .expect("Is DATABASE_URL set in .env file? Check where is .env file?");
  db_url_client
}