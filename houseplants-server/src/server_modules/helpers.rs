use dotenv::dotenv;
use std::env;

// get server port from .env file
pub fn get_server_port() -> String {
    // url of resource on server
    dotenv().ok();
    env::var("SERVER_PORT").expect("Is SERVER_PORT set in .env file? Check where is .env file?")
}

// get server port from .env file
pub fn get_db_url() -> String {
    // url of resource on server
    dotenv().ok();
    env::var("DATABASE_URL").expect("Is DATABASE_URL set in .env file? Check where is .env file?")
}
