use sqlx::postgres::PgPool;
use std::sync::Mutex;
pub struct AppState {
  pub server_is_running_message: String,
}