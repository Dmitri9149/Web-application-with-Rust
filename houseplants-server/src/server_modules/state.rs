use sqlx::postgres::PgPool;
pub struct AppState {
  pub server_is_running_message: String,
  pub db: PgPool, 
}