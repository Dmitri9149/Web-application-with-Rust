use dotenv::dotenv;
use std::env;

pub fn get_port () -> String {
    // url of resource on server 
    dotenv().ok();
    let port = env::var("SERVER_PORT")
      .expect("Is SERVER_PORT set in .env file? From what folder you start server (where in .env file)?");
    port
}