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