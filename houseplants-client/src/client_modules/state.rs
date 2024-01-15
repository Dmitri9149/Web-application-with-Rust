use sqlx::postgres::PgPool;
pub struct AppState {
  pub web_client_is_running_message: String,
  pub db: PgPool,
}