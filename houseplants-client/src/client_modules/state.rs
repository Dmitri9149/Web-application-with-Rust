use sqlx::postgres::PgPool;
use std::sync::Mutex;
pub struct AppState {
  pub web_client_is_running_message: String,
  pub db: PgPool,
}