use dotenv::dotenv;
use std::env;
use argon2::{self, Config};

// get server port from .env file 
pub fn get_server_port () -> String {
    dotenv().ok();
    let port = env::var("SERVER_PORT")
      .expect("Is SERVER_PORT set in .env file? Where in .env file?");
    port
}

// get host port from .env file 
pub fn get_host_port () -> String {
  dotenv().ok();
  let port = env::var("HOST_PORT")
    .expect("Is HOST_PORT set in .env file? Where in .env file?");
  port
}

// get db url from .env file 
pub fn get_db_url () -> String { 
  dotenv().ok();
  let db_url = env::var("DATABASE_URL")
    .expect("Is DATABASE_URL set in .env file? Where in .env file?");
  db_url
}

// Hashing the password to store it in DB 
pub fn hash_password(salt: &str, password: &str) -> String {
  let s = salt.as_bytes();
  let config = Config::default();
  argon2::hash_encoded(
    password.as_bytes(), s, &config)
    .unwrap()
}
