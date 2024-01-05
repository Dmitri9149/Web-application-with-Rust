use crate::state::AppState;
use actix_web::{web, HttpResponse};
use std::time::{SystemTime, UNIX_EPOCH};

// general route handler
pub async fn general_page_handler(app_state: web::Data<AppState>) ->
  HttpResponse {
    let web_client_is_running = &app_state.web_client_is_running_message;
    let time_now = SystemTime::now();
    let since_the_epoch = time_now
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
    let response = format!("{};  Current time is: {:?}; Duration since UNIX EPOCH is {:?}", 
      web_client_is_running, time_now, since_the_epoch);
    HttpResponse::Ok().json(&response)
}